#![feature(drain_filter, map_into_keys_values, slice_partition_dedup)]
#![allow(clippy::single_match, clippy::naive_bytecount)]
use actix_files::{Files, NamedFile};
use actix_service::fn_service;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    middleware, App, HttpServer,
};
use futures_util::future::{err, ok};
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
        let mut app = App::new()
            .data(config.clone())
            .data(games.clone())
            .wrap(middleware::Logger::default())
            .service(websocket::index);
        if let Some(path) = config.server.static_path.as_deref() {
            let mut index = path.to_owned();
            index.push(config.server.index.as_deref().unwrap_or("index.html"));
            let index_factory = fn_service(move |req: ServiceRequest| {
                let (req, _) = req.into_parts();
                let file = match NamedFile::open(&index) {
                    Ok(file) => file,
                    Err(error) => return err(error.into()),
                };
                let response = match file
                    .use_etag(true)
                    .use_last_modified(true)
                    .into_response(&req)
                {
                    Ok(response) => response,
                    Err(error) => return err(error),
                };
                ok(ServiceResponse::new(req, response))
            });
            app = app.default_service(
                Files::new("/", path)
                    .use_etag(true)
                    .use_last_modified(true)
                    .index_file(config.server.index.as_deref().unwrap_or("index.html"))
                    .default_handler(index_factory),
            );
        }
        app
    })
    .bind(address)?
    .run()
    .await
}
