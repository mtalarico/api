use mongodb::{
    bson::{self, Document},
    Cursor,
};

/// connects to instance at uri, specify options and credentials according to mongodb docs (https://www.mongodb.com/docs/manual/reference/connection-string/)
pub async fn connect(uri: &str) -> mongodb::error::Result<mongodb::Client> {
    let mut options = mongodb::options::ClientOptions::parse(uri).await?;
    options.app_name = Some("orphanage".to_string());
    let client = mongodb::Client::with_options(options)?;
    client
        .database("admin")
        .run_command(bson::doc! {"ping": 1}, None)
        .await?;
    log::debug!("Connected to {}", uri);
    Ok(client)
}

pub async fn aggregate(
    client: &mongodb::Client,
    ns: &mongodb::Namespace,
    pipeline: Vec<Document>,
) -> Cursor<Document> {
    client
        .database(ns.db.as_str())
        .collection::<Document>(ns.coll.as_str())
        .aggregate(pipeline, None)
        .await
        .unwrap()
}

pub async fn find(
    client: &mongodb::Client,
    ns: &mongodb::Namespace,
    query: Document,
) -> Cursor<Document> {
    client
        .database(ns.db.as_str())
        .collection::<Document>(ns.coll.as_str())
        .find(query, None)
        .await
        .unwrap()
}
