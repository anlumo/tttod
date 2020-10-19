use crate::{Error, Question};
use actix_web::rt::spawn;
use enum_iterator::IntoEnumIterator;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    StreamExt,
};
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use tttod_data::{ClientToServerMessage, GameState, Player, ServerToClientMessage};
use uuid::Uuid;

const MIN_PLAYERS: usize = 3;
const MAX_PLAYERS: usize = 5;
const QUESTIONS_PER_PLAYER: usize = 2;

#[derive(Debug, Clone)]
pub enum InternalMessage {
    Message {
        player_id: Uuid,
        message: ClientToServerMessage,
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
    clues: Vec<(Question, String)>,
}

impl GameManager {
    fn send_all(&self, message: ServerToClientMessage) -> Result<(), Error> {
        for (_, senders) in self.players.values() {
            for sender in senders {
                sender.unbounded_send(message.clone())?;
            }
        }
        Ok(())
    }
    fn push_state_all(&self, game_state: GameState) -> Result<(), Error> {
        let players: HashMap<_, _> = self
            .players
            .iter()
            .map(|(id, (player, _))| (*id, player.clone()))
            .collect();
        self.send_all(ServerToClientMessage::PushState {
            players,
            game_state,
            player_kick_votes: self.player_kick_votes.clone(),
        })
    }
    pub async fn run_game(receiver: UnboundedReceiver<InternalMessage>) {
        let mut instance = GameManager {
            receiver,
            players: HashMap::new(),
            player_kick_votes: HashMap::new(),
            clues: Vec::new(),
        };

        // wait for players
        if let Err(err) = instance.wait_for_players().await {
            log::error!("wait_for_players: {:?}", err);
            return;
        }
        // let players define the evil
        if let Err(err) = instance.define_evil().await {
            log::error!("define_evil: {:?}", err);
            return;
        }
        // let players create their character
        if let Err(err) = instance.create_character().await {
            log::error!("create_character: {:?}", err);
            return;
        }
        // enter the temple
        if let Err(err) = instance.enter_temple().await {
            log::error!("enter_temple: {:?}", err);
            return;
        }
        // face the ancient evil
        if let Err(err) = instance.face_ancient_evil().await {
            log::error!("face_ancient_evil: {:?}", err);
            return;
        }
    }

    async fn wait_for_players(&mut self) -> Result<(), Error> {
        while self.players.len() < MIN_PLAYERS
            || !self.players.values().all(|(player, _)| player.ready)
        {
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
                    } else if self.players.len() >= MAX_PLAYERS {
                        sender.unbounded_send(ServerToClientMessage::GameIsFull)?;
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
                Some(InternalMessage::Message { player_id, message }) => match message {
                    ClientToServerMessage::ReadyForGame => {
                        if let Some((player, _)) = self.players.get_mut(&player_id) {
                            player.ready = true;
                        }
                        self.push_state_all(GameState::PlayerSelection)?;
                    }
                    ClientToServerMessage::SetPlayerName { name } => {
                        if let Some((player, _)) = self.players.get_mut(&player_id) {
                            player.name = name;
                        }
                        self.push_state_all(GameState::PlayerSelection)?;
                    }
                    ClientToServerMessage::VoteKickPlayer {
                        player_id: other_player_id,
                    } => {
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
                    ClientToServerMessage::RevertVoteKickPlayer {
                        player_id: other_player_id,
                    } => {
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

    async fn define_evil(&mut self) -> Result<(), Error> {
        for (player, _) in self.players.values_mut() {
            player.ready = false;
        }
        self.player_kick_votes.clear();
        self.push_state_all(GameState::DefineEvil)?;

        let mut rng = rand::thread_rng();
        let mut questions: Vec<Question> = Question::into_enum_iter().collect();
        questions.shuffle(&mut rng);
        let questions_iter = questions.chunks_exact(QUESTIONS_PER_PLAYER);

        let mut player_questions: HashMap<Uuid, Vec<(Question, Option<String>)>> = self
            .players
            .keys()
            .zip(questions_iter)
            .map(|(id, questions)| (*id, questions.iter().map(|q| (*q, None)).collect()))
            .collect();

        for (player_id, (_, senders)) in &self.players {
            if let Some(questions) = player_questions.get(player_id) {
                let payload: Vec<(String, Option<String>)> = questions
                    .iter()
                    .map(|(question, _)| (format!("{}", question), None))
                    .collect();
                for sender in senders {
                    sender.unbounded_send(ServerToClientMessage::Questions {
                        questions: payload.clone(),
                    })?;
                }
            }
        }
        while !self.players.values().all(|(player, _)| player.ready) {
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
                            game_state: GameState::DefineEvil,
                            player_kick_votes: HashMap::new(),
                        })?;
                        if let Some(questions) = player_questions.get(&player_id) {
                            let payload = questions
                                .iter()
                                .map(|(question, answer)| (format!("{}", question), answer.clone()))
                                .collect();
                            sender.unbounded_send(ServerToClientMessage::Questions {
                                questions: payload,
                            })?;
                        }
                        senders.push(sender);
                    } else {
                        sender.unbounded_send(ServerToClientMessage::GameIsOngoing)?;
                    }
                }
                Some(InternalMessage::RemoveClient { player_id }) => {
                    if let Some((_, senders)) = self.players.get_mut(&player_id) {
                        senders.drain_filter(|sender| sender.is_closed());
                    }
                }
                Some(InternalMessage::Message { player_id, message }) => match message {
                    ClientToServerMessage::Answers { answers } => {
                        if let Some((player, _)) = self.players.get_mut(&player_id) {
                            if !player.ready {
                                if let Some(questions) = player_questions.get_mut(&player_id) {
                                    let mut answer_iter = answers.into_iter();
                                    for question in questions.iter_mut() {
                                        if let Some(answer) = answer_iter.next() {
                                            question.1 = Some(answer);
                                        }
                                    }
                                    if questions.iter().all(|(_, answer)| answer.is_some()) {
                                        player.ready = true;
                                        self.push_state_all(GameState::DefineEvil)?;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        self.clues.extend(
            player_questions
                .into_values()
                .flatten()
                .filter_map(|(question, answer)| answer.map(|answer| (question, answer))),
        );
        self.clues.shuffle(&mut rng);
        Ok(())
    }
    async fn create_character(&mut self) -> Result<(), Error> {
        for (player, _) in self.players.values_mut() {
            player.ready = false;
        }
        self.push_state_all(GameState::CharacterCreation)?;
        while !self.players.values().all(|(player, _)| player.ready) {
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
                            game_state: GameState::CharacterCreation,
                            player_kick_votes: HashMap::new(),
                        })?;
                        senders.push(sender);
                    } else {
                        sender.unbounded_send(ServerToClientMessage::GameIsOngoing)?;
                    }
                }
                Some(InternalMessage::RemoveClient { player_id }) => {
                    if let Some((_, senders)) = self.players.get_mut(&player_id) {
                        senders.drain_filter(|sender| sender.is_closed());
                    }
                }
                Some(InternalMessage::Message { player_id, message }) => match message {
                    ClientToServerMessage::SetCharacter { stats } => {
                        if let Some((player, _)) = self.players.get_mut(&player_id) {
                            if !player.ready
                                && stats.heroic > 0
                                && stats.booksmart > 0
                                && stats.streetwise > 0
                                && stats.heroic + stats.booksmart + stats.streetwise == 5
                                && !stats.name.is_empty()
                                && !stats.speciality.is_empty()
                                && !stats.reputation.is_empty()
                                && !stats.artifact_name.is_empty()
                                && !stats.artifact_origin.is_empty()
                            {
                                player.stats = Some(stats);
                                player.ready = true;
                            }
                        }
                        self.push_state_all(GameState::CharacterCreation)?;
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
    async fn enter_temple(&mut self) -> Result<(), Error> {
        for (player, _) in self.players.values_mut() {
            player.ready = false;
        }
        self.push_state_all(GameState::Game)?;
        let mut rng = rand::thread_rng();
        let mut gms: Vec<Uuid> = self.players.keys().cloned().collect();
        gms.shuffle(&mut rng);

        for gm in gms {
            self.send_all(ServerToClientMessage::DeclareGM { player_id: gm })?;

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
                            game_state: GameState::Game,
                            player_kick_votes: HashMap::new(),
                        })?;
                        senders.push(sender);
                    } else {
                        sender.unbounded_send(ServerToClientMessage::GameIsOngoing)?;
                    }
                }
                Some(InternalMessage::RemoveClient { player_id }) => {
                    if let Some((_, senders)) = self.players.get_mut(&player_id) {
                        senders.drain_filter(|sender| sender.is_closed());
                    }
                }
                Some(InternalMessage::Message { player_id, message }) => match message {
                    _ => {}
                },
            }
        }

        Ok(())
    }
    async fn face_ancient_evil(&mut self) -> Result<(), Error> {
        Ok(())
    }
}
