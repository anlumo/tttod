#![feature(drain_filter)]
use actix_web::{get, middleware, App, HttpServer, Responder};
use std::{
    collections::HashMap,
    net::SocketAddr,
    path::PathBuf,
    str::FromStr,
    sync::{Arc, Mutex},
};
use structopt::StructOpt;

mod config;
use config::Config;
mod error;
pub use error::Error;
mod clues;
pub use clues::Question;
mod game;
pub use game::Game;
mod websocket;

type Games = Arc<Mutex<HashMap<String, Game>>>;

#[derive(StructOpt, Debug)]
#[structopt(name = "tttod-server", about = "Backend for To the Temple of Doom!")]
struct Opt {
    #[structopt(short, long, parse(try_from_str))]
    /// Listening port, format address:port
    address: Option<SocketAddr>,
    #[structopt(short, long, parse(try_from_str), default_value = "config.yaml")]
    /// Path to the config file
    config: PathBuf,
    #[structopt(short, long)]
    /// URL base for redirects
    base: Option<String>,
}

#[get("/")]
async fn index() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    let (config, logging_config) = match Config::parse(&opt.config) {
        Err(err) => {
            eprintln!("Error in config file `{}`: {}", opt.config.display(), err);
            std::process::exit(-1);
        }
        Ok(config) => config,
    };
    if let Err(e) = log4rs::init_config(logging_config) {
        eprintln!("log4rs: {}", e);
        std::process::exit(-1);
    }
    let address = opt
        .address
        .or(config.server.address)
        .unwrap_or_else(|| SocketAddr::from_str("127.0.0.1:8081").unwrap());

    let games: Games = Arc::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .data(config.clone())
            .data(games.clone())
            .wrap(middleware::Logger::default())
            .service(websocket::index)
            .service(index)
    })
    .bind(address)?
    .run()
    .await
}
