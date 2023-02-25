mod api;
mod cli;
mod db;
mod translation;
mod util;

use api::{sequence_query, sequence_search};
use axum::{routing::get, Router};

use crate::translation::query_definition::QueryDefinitions;

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Client,
    pub definitions: QueryDefinitions,
}

const DEFINITIONS_PATH: &str = "./query_definitions";

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    util::init_logging();

    let args = cli::args();
    let state = init_state(args).await;

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app(state).into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn init_state(args: cli::Args) -> AppState {
    log::info!("initializing");
    
    let definitions = QueryDefinitions::parse_all_in_dir(DEFINITIONS_PATH).await;
    log::info!("{:#?}", &definitions);
    let db = db::connect(args.uri.as_str()).await.unwrap();
    AppState { db, definitions }
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/sequence/search", get(sequence_search::search))
        .route("/sequence/query", get(sequence_query::query))
        .with_state(state)
}
