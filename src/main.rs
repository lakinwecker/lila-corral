extern crate clap;
extern crate dotenv;
extern crate pretty_env_logger;
#[macro_use] extern crate log;

use dotenv::dotenv;
use structopt::StructOpt;
use axum::{
    routing::get,
    Router,
};

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

#[tokio::main]
async fn main() {

    pretty_env_logger::init();

    debug!("Reading dotenv...");
    dotenv().ok();

    let opts = CorralWebserverOpts::from_args();

    // build our application with a single route
    let app = Router::new().route("/", get(|| async {
        info!("Some logging");
        "Hello, World!"
    }));

    // run it with hyper on localhost:3000
    let uri = format!("{}:{}", opts.host, opts.port);
    info!("Corral listening on {}", uri);
    axum::Server::bind(&uri.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
