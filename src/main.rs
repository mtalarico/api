mod api;
mod cli;
mod db;
mod model;
mod translation;
mod util;

use std::collections::HashMap;

use api::{sequence_query, sequence_search};
use axum::{routing::get, Router};
use translation::translation::Translations;

#[derive(Clone)]
pub struct AppState {
    pub db: mongodb::Client,
    pub translations: Translations,
}

const SEQ_PATH: &str = "./src/api/sequence_mapping.json";

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
    let translations = Translations::parse_all_in_dir(TRANSLATION_DEFINITION_PATH).await;
    let db = db::connect(args.uri.as_str()).await.unwrap();
    AppState { db, translations }
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/sequence/search", get(sequence_search::search))
        .route("/sequence/query", get(sequence_query::query))
        .with_state(state)
}
