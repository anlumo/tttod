use crate::{Error, Question};
use actix_web::rt::spawn;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    StreamExt,
};
use std::collections::{HashMap, HashSet};
use tttod_data::{ClientToServerMessage, GameState, MessageFrame, Player, ServerToClientMessage};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum InternalMessage {
    Message {
        player_id: Uuid,
        message: MessageFrame,
    },
    AddClient {
        player_id: Uuid,
        sender: UnboundedSender<ServerToClientMessage>,
    },
    RemoveClient {
        player_id: Uuid,
    },
}

#[derive(Debug, Clone)]
pub struct Game(UnboundedSender<InternalMessage>);

impl AsRef<UnboundedSender<InternalMessage>> for Game {
    fn as_ref(&self) -> &UnboundedSender<InternalMessage> {
        &self.0
    }
}

impl Default for Game {
    fn default() -> Self {
        let (sender, receiver) = unbounded();
        spawn(GameManager::run_game(receiver));
        Game(sender)
    }
}

struct GameManager {
    receiver: UnboundedReceiver<InternalMessage>,
    players: HashMap<Uuid, (Player, Vec<UnboundedSender<ServerToClientMessage>>)>,
    player_kick_votes: HashMap<Uuid, HashSet<Uuid>>,
    clues: HashMap<Question, String>,
}

impl GameManager {
    fn push_state_all(&self, game_state: GameState) -> Result<(), Error> {
        let all_players: HashMap<_, _> = self
            .players
            .iter()
            .map(|(id, (player, _))| (*id, player.clone()))
            .collect();
        for (_, senders) in self.players.values() {
            for sender in senders {
                sender.unbounded_send(ServerToClientMessage::PushState {
                    players: all_players.clone(),
                    game_state,
                    player_kick_votes: self.player_kick_votes.clone(),
                })?;
            }
        }
        Ok(())
    }
    pub async fn run_game(receiver: UnboundedReceiver<InternalMessage>) {
        let mut instance = GameManager {
            receiver,
            players: HashMap::new(),
            player_kick_votes: HashMap::new(),
            clues: HashMap::new(),
        };

        // wait for players
        if let Err(err) = instance.wait_for_players().await {
            log::error!("wait_for_players: {:?}", err);
            return;
        }
    }
    async fn wait_for_players(&mut self) -> Result<(), Error> {
        while self.players.is_empty() || !self.players.values().all(|(player, _)| player.ready) {
            match self.receiver.next().await {
                None => {
                    log::error!("Game failed");
                    return Err(Error::NoPlayers);
                }
                Some(InternalMessage::AddClient { player_id, sender }) => {
                    let all_players: HashMap<_, _> = self
                        .players
                        .iter()
                        .map(|(id, (player, _))| (*id, player.clone()))
                        .collect();
                    if let Some((_, senders)) = self.players.get_mut(&player_id) {
                        sender.unbounded_send(ServerToClientMessage::PushState {
                            players: all_players.clone(),
                            game_state: GameState::PlayerSelection,
                            player_kick_votes: self.player_kick_votes.clone(),
                        })?;
                        senders.push(sender);
                    } else {
                        self.players
                            .insert(player_id, (Player::default(), vec![sender]));
                        self.push_state_all(GameState::PlayerSelection)?;
                    }
                }
                Some(InternalMessage::RemoveClient { player_id }) => {
                    if let Some((_, senders)) = self.players.get_mut(&player_id) {
                        senders.drain_filter(|sender| sender.is_closed());
                    }
                }
                Some(InternalMessage::Message { player_id, message }) => match message.msg {
                    ClientToServerMessage::ReadyForGame => {
                        if let Some((player, _)) = self.players.get_mut(&player_id) {
                            player.ready = true;
                        }
                        self.push_state_all(GameState::PlayerSelection)?;
                    }
                    ClientToServerMessage::SetPlayerName(name) => {
                        if let Some((player, _)) = self.players.get_mut(&player_id) {
                            player.name = name;
                        }
                        self.push_state_all(GameState::PlayerSelection)?;
                    }
                    ClientToServerMessage::VoteKickPlayer(other_player_id) => {
                        if player_id != other_player_id {
                            let votes = self.player_kick_votes.entry(other_player_id).or_default();
                            votes.insert(player_id);
                            let online_voters: HashSet<_> = self
                                .players
                                .iter()
                                .filter_map(|(id, (_, senders))| {
                                    if senders.is_empty() || id == &other_player_id {
                                        None
                                    } else {
                                        Some(*id)
                                    }
                                })
                                .collect();
                            let votes = votes.intersection(&online_voters).count();
                            let voting_player_count = self
                                .players
                                .keys()
                                .filter(|id| online_voters.contains(*id))
                                .count();
                            if votes >= voting_player_count {
                                self.players.remove(&other_player_id);
                                for player_kick_votes in self.player_kick_votes.values_mut() {
                                    player_kick_votes.remove(&other_player_id);
                                }
                            }
                            self.push_state_all(GameState::PlayerSelection)?;
                        }
                    }
                    ClientToServerMessage::RevertVoteKickPlayer(other_player_id) => {
                        if let Some(votes) = self.player_kick_votes.get_mut(&other_player_id) {
                            votes.remove(&player_id);
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
}
