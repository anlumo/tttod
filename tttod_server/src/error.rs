use actix_web::{http::StatusCode, HttpResponse};
use futures::channel::mpsc::SendError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    MutexPoisoned,
    Actix(actix_web::Error),
    GameIsFull,
    UnableToJoin(SendError),
    SendError(SendError),
    NoPlayers,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MutexPoisoned => write!(f, "Mutex poisoned"),
            Self::Actix(err) => err.fmt(f),
            Self::GameIsFull => write!(f, "Game is full"),
            Self::UnableToJoin(err) => err.fmt(f),
            Self::SendError(err) => err.fmt(f),
            Self::NoPlayers => write!(f, "No players left in game"),
        }
    }
}

impl std::error::Error for Error {}

impl From<actix_web::Error> for Error {
    fn from(err: actix_web::Error) -> Self {
        Self::Actix(err)
    }
}

impl<T> From<futures::channel::mpsc::TrySendError<T>> for Error {
    fn from(err: futures::channel::mpsc::TrySendError<T>) -> Self {
        Self::SendError(err.into_send_error())
    }
}

impl actix_web::error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::GameIsFull => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::GameIsFull => HttpResponse::BadRequest().body("Game is full"),
            Self::UnableToJoin(_) => {
                HttpResponse::InternalServerError().body("Unable to join game")
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
