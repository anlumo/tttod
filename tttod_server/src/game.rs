use crate::{Error, Question};
use actix_web::rt::spawn;
use enum_iterator::IntoEnumIterator;
use futures::{
    channel::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    StreamExt,
};
use rand::{seq::SliceRandom, RngCore};
use std::collections::{HashMap, HashSet};
use tttod_data::{
    ArtifactBoon, Attribute, Challenge, ChallengeResult, ClientToServerMessage, Condition,
    GameState, MentalCondition, Player, ServerToClientMessage, FAILURES_NEEDED, SUCCESSES_NEEDED,
};
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
    fn send_all_f(&mut self, mut f: impl FnMut(Uuid) -> Option<ServerToClientMessage>) {
        for (&player_id, (_, senders)) in self.players.iter_mut() {
            if let Some(msg) = f(player_id) {
                senders.drain_filter(move |sender| sender.unbounded_send(msg.clone()).is_err());
            }
        }
    }
    fn send_all(&mut self, message: ServerToClientMessage) {
        self.send_all_f(|_| Some(message.clone()));
    }
    fn send_to_client(
        &mut self,
        player_id: Uuid,
        client_idx: usize,
        message: ServerToClientMessage,
    ) {
        if let Some((_, senders)) = self.players.get_mut(&player_id) {
            let fail = if let Some(client) = senders.get(client_idx) {
                client.unbounded_send(message).is_err()
            } else {
                false
            };
            if fail {
                senders.remove(client_idx);
            }
        }
    }
    fn send_to(&mut self, player_id: Uuid, message: ServerToClientMessage) {
        if let Some((_, senders)) = self.players.get_mut(&player_id) {
            senders.drain_filter(|sender| sender.unbounded_send(message.clone()).is_err());
        }
    }
    fn known_clues(&self, room_idx: usize) -> Vec<String> {
        self.clues[0..room_idx]
            .iter()
            .map(|(_, clue)| clue.clone())
            .collect()
    }
    fn push_state_all(&mut self, game_state: GameState) {
        let known_clues = if let GameState::Room { room_idx, .. } = game_state {
            self.known_clues(room_idx)
        } else {
            Vec::new()
        };
        let players: HashMap<_, _> = self
            .players
            .iter()
            .map(|(id, (player, _))| (*id, player.clone()))
            .collect();
        self.send_all(ServerToClientMessage::PushState {
            players,
            game_state,
            player_kick_votes: self.player_kick_votes.clone(),
            known_clues,
        });
    }
    fn roll_d6(count: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        (0..count)
            .map(|_| (6.0 * ((rng.next_u32() as f64) / (std::u32::MAX as f64))) as u8 + 1)
            .collect()
    }
    fn possessed_dice(dice: &[u8]) -> bool {
        let ones = dice.iter().filter(|die| **die == 1).count();
        if ones > 1 {
            true
        } else {
            let twos = dice.iter().filter(|die| **die == 2).count();
            twos > 1
        }
    }
    fn check_success(dice: &[u8], artifact: Option<ArtifactBoon>) -> bool {
        match artifact {
            Some(ArtifactBoon::SuccessOnFive) => dice.contains(&5),
            Some(ArtifactBoon::SuccessOnDoubles) => {
                let mut results = dice.to_vec();
                results.sort_unstable();
                let (_, duplicates) = results.partition_dedup();
                !duplicates.is_empty()
            }
            _ => dice.contains(&6),
        }
    }
    /// Does not check whether the player has already used the artifact previously!
    fn check_can_use_artifact(dice: &[u8], artifact_boon: ArtifactBoon) -> bool {
        let success = Self::check_success(dice, None);
        let possession = Self::possessed_dice(&dice);
        if success && !possession {
            false // no point in using it
        } else if possession && artifact_boon == ArtifactBoon::Reroll {
            true
        } else if !success {
            // check whether the artifact could make a difference
            match artifact_boon {
                ArtifactBoon::SuccessOnFive if !dice.contains(&5) => false,
                ArtifactBoon::SuccessOnDoubles => {
                    let mut results = dice.to_vec();
                    results.sort_unstable();
                    let (_, duplicates) = results.partition_dedup();
                    !duplicates.is_empty()
                }
                _ => true,
            }
        } else {
            false
        }
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
        // character introduction
        if let Err(err) = instance.introduce_characters().await {
            log::error!("introduce_characters: {:?}", err);
            return;
        }
        // enter the temple
        match instance.enter_temple().await {
            Err(err) => {
                log::error!("enter_temple: {:?}", err);
                return;
            }
            Ok(success) => {
                if success {
                    // face the ancient evil
                    match instance.face_ancient_evil().await {
                        Err(err) => {
                            log::error!("face_ancient_evil: {:?}", err);
                        }
                        Ok(success) => {
                            if let Err(err) = instance.end(success).await {
                                log::error!("end: {:?}", err);
                            }
                        }
                    }
                } else if let Err(err) = instance.end(false).await {
                    log::error!("failed: {:?}", err);
                }
            }
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
                        let client_idx = senders.len();
                        senders.push(sender);
                        self.send_to_client(
                            player_id,
                            client_idx,
                            ServerToClientMessage::PushState {
                                players: all_players.clone(),
                                game_state: GameState::PlayerSelection,
                                player_kick_votes: self.player_kick_votes.clone(),
                                known_clues: Vec::new(),
                            },
                        );
                    } else if self.players.len() >= MAX_PLAYERS {
                        sender.unbounded_send(ServerToClientMessage::GameIsFull)?;
                    } else {
                        self.players
                            .insert(player_id, (Player::default(), vec![sender]));
                        self.push_state_all(GameState::PlayerSelection);
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
                        self.push_state_all(GameState::PlayerSelection);
                    }
                    ClientToServerMessage::SetPlayerName { name } => {
                        if let Some((player, _)) = self.players.get_mut(&player_id) {
                            player.name = name;
                        }
                        self.push_state_all(GameState::PlayerSelection);
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
                            self.push_state_all(GameState::PlayerSelection);
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
        self.push_state_all(GameState::DefineEvil);

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

        self.send_all_f(|player_id| {
            player_questions.get(&player_id).map(|questions| {
                let payload: Vec<(String, Option<String>)> = questions
                    .iter()
                    .map(|(question, _)| (format!("{}", question), None))
                    .collect();
                ServerToClientMessage::Questions { questions: payload }
            })
        });
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
                        let client_idx = senders.len();
                        senders.push(sender);
                        self.send_to_client(
                            player_id,
                            client_idx,
                            ServerToClientMessage::PushState {
                                players: all_players.clone(),
                                game_state: GameState::DefineEvil,
                                player_kick_votes: HashMap::new(),
                                known_clues: Vec::new(),
                            },
                        );

                        if let Some(questions) = player_questions.get(&player_id) {
                            let payload = questions
                                .iter()
                                .map(|(question, answer)| (format!("{}", question), answer.clone()))
                                .collect();
                            self.send_to_client(
                                player_id,
                                client_idx,
                                ServerToClientMessage::Questions { questions: payload },
                            );
                        }
                    } else {
                        sender
                            .unbounded_send(ServerToClientMessage::GameIsOngoing)
                            .ok();
                        sender.close_channel();
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
                                            if !answer.is_empty() {
                                                question.1 = Some(answer);
                                            } else {
                                                question.1 = None;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ClientToServerMessage::ReadyForGame => {
                        let mut ready = false;
                        if let Some((player, _)) = self.players.get_mut(&player_id) {
                            if !player.ready {
                                if let Some(questions) = player_questions.get(&player_id) {
                                    ready = questions.iter().all(|(_, answer)| {
                                        answer.as_ref().filter(|a| !a.is_empty()).is_some()
                                    });
                                    if ready {
                                        player.ready = true;
                                    }
                                }
                            }
                        }
                        if ready {
                            self.push_state_all(GameState::DefineEvil);
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
        self.push_state_all(GameState::CharacterCreation);
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
                        let client_idx = senders.len();
                        senders.push(sender);
                        self.send_to_client(
                            player_id,
                            client_idx,
                            ServerToClientMessage::PushState {
                                players: all_players.clone(),
                                game_state: GameState::CharacterCreation,
                                player_kick_votes: HashMap::new(),
                                known_clues: Vec::new(),
                            },
                        );
                    } else {
                        sender
                            .unbounded_send(ServerToClientMessage::GameIsOngoing)
                            .ok();
                        sender.close_channel();
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
                            if !player.ready {
                                player.stats = Some(stats);
                            }
                        }
                        self.push_state_all(GameState::CharacterCreation);
                    }
                    ClientToServerMessage::ReadyForGame => {
                        if let Some((player, _)) = self.players.get_mut(&player_id) {
                            if let Some(stats) = &player.stats {
                                let heroic = stats
                                    .attributes
                                    .get(&Attribute::Heroic)
                                    .cloned()
                                    .unwrap_or(0);
                                let booksmart = stats
                                    .attributes
                                    .get(&Attribute::Booksmart)
                                    .cloned()
                                    .unwrap_or(0);
                                let streetwise = stats
                                    .attributes
                                    .get(&Attribute::Streetwise)
                                    .cloned()
                                    .unwrap_or(0);
                                if heroic > 0
                                    && booksmart > 0
                                    && streetwise > 0
                                    && heroic + booksmart + streetwise == 5
                                    && !stats.name.is_empty()
                                    && !stats.artifact_name.is_empty()
                                    && !stats.artifact_origin.is_empty()
                                {
                                    player.ready = true;
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
    async fn introduce_characters(&mut self) -> Result<(), Error> {
        for (player, _) in self.players.values_mut() {
            player.ready = false;
        }
        self.push_state_all(GameState::CharacterIntroduction);

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
                        let client_idx = senders.len();
                        senders.push(sender);
                        self.send_to_client(
                            player_id,
                            client_idx,
                            ServerToClientMessage::PushState {
                                players: all_players.clone(),
                                game_state: GameState::CharacterIntroduction,
                                player_kick_votes: HashMap::new(),
                                known_clues: Vec::new(),
                            },
                        );
                    } else {
                        sender
                            .unbounded_send(ServerToClientMessage::GameIsOngoing)
                            .ok();
                        sender.close_channel();
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
                        self.push_state_all(GameState::CharacterIntroduction);
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
    async fn enter_temple(&mut self) -> Result<bool, Error> {
        let mut rng = rand::thread_rng();
        let mut gms: Vec<Uuid> = self.players.keys().cloned().collect();
        gms.shuffle(&mut rng);

        for (room, gm) in gms.into_iter().enumerate() {
            let mut successes = 0;
            let mut failures = 0;
            let mut clue = self.clues[room].1.clone();
            for (&player_id, (player, _)) in self.players.iter_mut() {
                player.ready = player_id != gm;
            }
            self.push_state_all(GameState::Room {
                room_idx: room,
                gm,
                successes,
                failures,
            });
            self.send_to(gm, ServerToClientMessage::PushClue { clue: clue.clone() });

            let mut current_challenge: Option<Challenge> = None;
            let mut current_challenge_result: Option<Vec<u8>> = None;
            let mut current_artifact_used: Option<ArtifactBoon> = None;

            while successes < SUCCESSES_NEEDED {
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
                            let client_idx = senders.len();
                            senders.push(sender);
                            self.send_to_client(
                                player_id,
                                client_idx,
                                ServerToClientMessage::PushState {
                                    players: all_players.clone(),
                                    game_state: GameState::Room {
                                        room_idx: room,
                                        gm,
                                        successes,
                                        failures,
                                    },
                                    player_kick_votes: HashMap::new(),
                                    known_clues: self.known_clues(room),
                                },
                            );
                            if player_id == gm {
                                self.send_to(
                                    player_id,
                                    ServerToClientMessage::PushClue { clue: clue.clone() },
                                );
                            }
                            if let Some(current_challenge) = &current_challenge {
                                if current_challenge.player_id == player_id {
                                    let (artifact_boon, artifact_used) = self
                                        .players
                                        .get(&player_id)
                                        .map(|(player, _)| {
                                            (
                                                player
                                                    .stats
                                                    .as_ref()
                                                    .map(|stats| stats.artifact_boon),
                                                player.artifact_used,
                                            )
                                        })
                                        .unwrap_or((None, true));
                                    self.send_to_client(
                                        player_id,
                                        client_idx,
                                        ServerToClientMessage::ReceivedChallenge(
                                            current_challenge.clone(),
                                        ),
                                    );
                                    if let Some(challenge_result) = &current_challenge_result {
                                        let can_use_artifact = artifact_boon
                                            .map(|artifact_boon| {
                                                !artifact_used
                                                    && Self::check_can_use_artifact(
                                                        &challenge_result,
                                                        artifact_boon,
                                                    )
                                            })
                                            .unwrap_or(false);
                                        self.send_to_client(
                                            player_id,
                                            client_idx,
                                            ServerToClientMessage::ChallengeResult(
                                                ChallengeResult {
                                                    possession: Self::possessed_dice(
                                                        &challenge_result,
                                                    ),
                                                    success: Self::check_success(
                                                        &challenge_result,
                                                        current_artifact_used,
                                                    ),
                                                    can_use_artifact,
                                                    rolls: challenge_result.clone(),
                                                },
                                            ),
                                        );
                                    }
                                }
                            }
                        } else {
                            sender
                                .unbounded_send(ServerToClientMessage::GameIsOngoing)
                                .ok();
                            sender.close_channel();
                        }
                    }
                    Some(InternalMessage::RemoveClient { player_id }) => {
                        if let Some((_, senders)) = self.players.get_mut(&player_id) {
                            senders.drain_filter(|sender| sender.is_closed());
                        }
                    }
                    Some(InternalMessage::Message { player_id, message }) => match message {
                        ClientToServerMessage::RejectClue if player_id == gm => {
                            if room > 0 && self.clues.len() > self.players.len() {
                                // clue doesn't fit with existing lore, remove it
                                self.clues.remove(room);
                                clue = self.clues[room].1.clone();
                                self.send_to(
                                    gm,
                                    ServerToClientMessage::PushClue { clue: clue.clone() },
                                );
                            } else {
                                // either there's no existing lore yet, or we don't have any more clues left to discard
                                self.send_to(
                                    player_id,
                                    ServerToClientMessage::ClueRejectionRejected,
                                );
                            }
                        }
                        ClientToServerMessage::OfferChallenge { challenge }
                            if player_id == gm && challenge.player_id != gm =>
                        {
                            if let Some((player, _)) = self.players.get(&player_id) {
                                if player.condition != Condition::Dead
                                    && player.mental_condition != MentalCondition::Possessed
                                {
                                    current_challenge = Some(challenge.clone());
                                    self.send_to(
                                        challenge.player_id,
                                        ServerToClientMessage::ReceivedChallenge(challenge),
                                    );
                                }
                            }
                        }
                        ClientToServerMessage::ChallengeAccepted => {
                            if let Some((player, _)) = self.players.get(&player_id) {
                                if let Some(challenge) = &current_challenge {
                                    if challenge.player_id == player_id {
                                        let dice_count = if let Some((player, _)) =
                                            self.players.get(&player_id)
                                        {
                                            player
                                                .stats
                                                .as_ref()
                                                .unwrap()
                                                .attributes
                                                .get(&challenge.attribute)
                                                .unwrap()
                                                + if challenge.speciality_applies { 1 } else { 0 }
                                                + if challenge.reputation_applies { 1 } else { 0 }
                                        } else {
                                            0
                                        };
                                        let mut can_use_artifact = false;
                                        let results = Self::roll_d6(dice_count as _);
                                        let success = Self::check_success(&results, None);
                                        let possession = Self::possessed_dice(&results);
                                        if success && !possession {
                                            successes += 1;
                                            current_challenge = None;
                                        } else if !player.artifact_used {
                                            let artifact_boon =
                                                player.stats.as_ref().unwrap().artifact_boon;
                                            if possession && artifact_boon == ArtifactBoon::Reroll {
                                                can_use_artifact = true;
                                            } else if !success {
                                                // check whether the artifact could make a difference
                                                match artifact_boon {
                                                    ArtifactBoon::SuccessOnFive
                                                        if !results.contains(&5) =>
                                                    {
                                                        // can't use artifact
                                                    }
                                                    ArtifactBoon::SuccessOnDoubles => {
                                                        let mut results = results.clone();
                                                        results.sort_unstable();
                                                        let (_, duplicates) =
                                                            results.partition_dedup();
                                                        can_use_artifact = !duplicates.is_empty();
                                                    }
                                                    _ => {
                                                        can_use_artifact = true;
                                                    }
                                                }
                                            }
                                        }
                                        if !success
                                            || (!player.artifact_used
                                                && player.stats.as_ref().unwrap().artifact_boon
                                                    == ArtifactBoon::Reroll
                                                && possession)
                                        {
                                            // the player can either use the artifact (if possible) or choose to
                                            // take a hit to avoid the failure
                                            // if it's just about possession, only the reroll artifact can help
                                            current_challenge_result = Some(results.clone());
                                        }
                                        self.send_all(ServerToClientMessage::ChallengeResult(
                                            ChallengeResult {
                                                rolls: results,
                                                success,
                                                possession,
                                                can_use_artifact,
                                            },
                                        ));
                                    }
                                }
                            }
                        }
                        ClientToServerMessage::ChallengeRejected => {
                            if (player_id == gm
                                || current_challenge
                                    .as_ref()
                                    .map(|challenge| challenge.player_id)
                                    == Some(player_id))
                                && current_challenge_result.is_none()
                            {
                                current_challenge = None;
                                self.send_to(player_id, ServerToClientMessage::AbortedChallenge);
                            }
                        }
                        ClientToServerMessage::AcceptFate => {
                            if let Some(current_challenge_result) = current_challenge_result.take()
                            {
                                if Self::check_success(
                                    &current_challenge_result,
                                    current_artifact_used.take(),
                                ) {
                                    successes += 1;
                                } else {
                                    failures += 1;
                                }
                                if Self::possessed_dice(&current_challenge_result) {
                                    if let Some((player, _)) = self.players.get_mut(&player_id) {
                                        player.mental_condition =
                                            player.mental_condition.take_hit();
                                    }
                                }
                                current_challenge = None;
                            }
                        }
                        ClientToServerMessage::TakeWound => {
                            if let Some((player, _)) = self.players.get_mut(&player_id) {
                                if let Some(challenge_result) = current_challenge_result.take() {
                                    player.condition = player.condition.take_hit();
                                    if Self::possessed_dice(&challenge_result) {
                                        player.mental_condition =
                                            player.mental_condition.take_hit();
                                    }
                                    successes += 1;
                                    current_challenge = None;
                                    current_artifact_used = None;
                                }
                            }
                        }
                        ClientToServerMessage::UseArtifact => {
                            if let Some(challenge_result) = current_challenge_result.take() {
                                if let Some((player, _)) = self.players.get_mut(&player_id) {
                                    player.artifact_used = true;
                                    let results = match player.stats.as_ref().unwrap().artifact_boon
                                    {
                                        ArtifactBoon::Reroll => {
                                            Self::roll_d6(challenge_result.len())
                                        }
                                        ArtifactBoon::RollWithPlusTwo => challenge_result
                                            .into_iter()
                                            .chain(Self::roll_d6(2).into_iter())
                                            .collect(),
                                        ArtifactBoon::SuccessOnFive
                                        | ArtifactBoon::SuccessOnDoubles => challenge_result,
                                    };
                                    let success = Self::check_success(
                                        &results,
                                        Some(player.stats.as_ref().unwrap().artifact_boon),
                                    );
                                    let possession = Self::possessed_dice(&results);
                                    if success {
                                        successes += 1;
                                        current_challenge_result = None;
                                        current_challenge = None;
                                    } else {
                                        current_artifact_used =
                                            Some(player.stats.as_ref().unwrap().artifact_boon);
                                    }
                                    self.send_all(ServerToClientMessage::ChallengeResult(
                                        ChallengeResult {
                                            possession,
                                            rolls: results,
                                            success,
                                            can_use_artifact: false,
                                        },
                                    ));

                                    self.push_state_all(GameState::Room {
                                        room_idx: room,
                                        gm,
                                        successes,
                                        failures,
                                    });
                                }
                            }
                        }
                        ClientToServerMessage::FakeFailure => {
                            failures += 1;
                            self.push_state_all(GameState::Room {
                                room_idx: room,
                                gm,
                                successes,
                                failures,
                            });
                        }
                        ClientToServerMessage::FakeSuccess => {
                            successes += 1;
                            self.push_state_all(GameState::Room {
                                room_idx: room,
                                gm,
                                successes,
                                failures,
                            });
                        }
                        _ => {}
                    },
                }
                if failures >= FAILURES_NEEDED {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
    async fn face_ancient_evil(&mut self) -> Result<bool, Error> {
        Ok(false)
    }
    async fn end(&mut self, victory: bool) -> Result<(), Error> {
        let game_state = if victory {
            GameState::Victory
        } else {
            GameState::Failure
        };
        for (player, _) in self.players.values_mut() {
            player.ready = false;
        }
        self.push_state_all(game_state);
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
                        let client_idx = senders.len();
                        senders.push(sender);
                        self.send_to_client(
                            player_id,
                            client_idx,
                            ServerToClientMessage::PushState {
                                players: all_players.clone(),
                                game_state,
                                player_kick_votes: HashMap::new(),
                                known_clues: Vec::new(),
                            },
                        );
                    } else {
                        sender
                            .unbounded_send(ServerToClientMessage::GameIsOngoing)
                            .ok();
                        sender.close_channel();
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
                        self.push_state_all(game_state);
                    }
                    _ => {}
                },
            }
        }
        Ok(())
    }
}
