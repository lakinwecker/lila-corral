extern crate clap;
extern crate dotenv;
extern crate thiserror;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use dotenv::dotenv;
use structopt::StructOpt;
use std::fmt;
use axum::{
    body,
    routing::get,
    Router,
    response::Response,
    http::StatusCode,
};
use thiserror::Error;

#[derive(Debug, StructOpt, Clone)]
struct DatabaseOpts {
    #[structopt(long, env = "LILA_CORRAL_MONGO_URI")]
    mongo_uri: String,

    #[structopt(long, env = "LILA_CORRAL_MONGO_DATABASE")]
    mongo_database: String,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Runs the main lila-deepq webserver.")]
struct CorralWebserverOpts {
    #[structopt(short, long, env = "LILA_CORRAL_WEBSERVER_HOST")]
    host: String,

    #[structopt(short, long, env = "LILA_CORRAL_WEBSERVER_PORT")]
    port: u16,

    #[structopt(flatten)]
    database_opts: DatabaseOpts,
}

#[derive(Error, Debug)]
pub enum Error {
    ReqwestError(#[from] reqwest::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ReqwestError(err) => write!(f, "Reqwest request: {}", err),
        }
    }
}

impl axum::response::IntoResponse for Error {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(body::boxed(body::Full::from(self.to_string())))
            .unwrap()
    }
}


async fn handler() -> Result<String, Error> {
    info!("Some logging");
    let body = reqwest::get("https://wttr.in")
        .await?
        .text()
        .await?;
    Ok(body)
}

#[tokio::main]
async fn main() {

    pretty_env_logger::init();

    debug!("Reading dotenv...");
    dotenv().ok();

    let opts = CorralWebserverOpts::from_args();
    // build our application with a single route
    let app = Router::new().route(
        "/",
        get(handler)
    );

    // run it with hyper on localhost:3000
    let uri = format!("{}:{}", opts.host, opts.port);
    info!("Corral listening on {}", uri);
    axum::Server::bind(&uri.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
