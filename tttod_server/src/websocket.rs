use crate::{game::InternalMessage, Error, Game};
use actix::{Actor, AsyncContext, StreamHandler};
use actix_web::{get, web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use tttod_data::ServerToClientMessage;
use uuid::Uuid;

#[derive(Debug)]
struct GameSocket {
    player_id: Uuid,
    game: Game,
    receiver: Option<UnboundedReceiver<ServerToClientMessage>>,
}

impl Actor for GameSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GameSocket {
    fn started(&mut self, ctx: &mut Self::Context) {
        if let Some(receiver) = self.receiver.take() {
            ctx.add_stream(receiver);
        }
    }
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                if let Ok(msg) = serde_json::from_str(&text) {
                    if let Err(err) = self.game.as_ref().unbounded_send(InternalMessage::Message {
                        player_id: self.player_id,
                        message: msg,
                    }) {
                        log::error!("Failed sending message to game: {:?}", err);
                        ctx.close(None);
                    }
                }
            }
            Ok(ws::Message::Binary(_)) => {
                log::error!("Received unknown binary message!");
            }
            _ => (),
        }
    }
}

impl StreamHandler<ServerToClientMessage> for GameSocket {
    fn handle(&mut self, msg: ServerToClientMessage, ctx: &mut Self::Context) {
        match msg.into_json() {
            Ok(txt) => {
                ctx.text(txt);
            }
            Err(err) => {
                log::error!("Failed serializing message: {:?}", err);
            }
        }
    }
}

#[get("/api/{game_name}/{player_id}/ws")]
pub async fn index(
    web::Path((game_name, player_id)): web::Path<(String, Uuid)>,
    games: web::Data<crate::Games>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let mut game = {
        games
            .lock()
            .map_err(|_| Error::MutexPoisoned)?
            .entry(game_name.clone())
            .or_default()
            .clone()
    };
    let (sender, receiver) = unbounded();
    if let Err(err) = game
        .as_ref()
        .unbounded_send(InternalMessage::AddClient { player_id, sender })
    {
        if err.is_disconnected() {
            game = Game::default();
            games
                .lock()
                .map_err(|_| Error::MutexPoisoned)?
                .insert(game_name, game.clone());
            game.as_ref()
                .unbounded_send(err.into_inner())
                .map_err(|err| Error::UnableToJoin(err.into_send_error()))?;
        } else {
            return Err(Error::UnableToJoin(err.into_send_error()));
        }
    }
    Ok(ws::start(
        GameSocket {
            player_id,
            game,
            receiver: Some(receiver),
        },
        &req,
        stream,
    )?)
}
