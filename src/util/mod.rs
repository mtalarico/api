use mongodb::bson::{doc, Bson, Document};

pub mod date_wrapper;

pub fn search_range<T: Into<Bson> + std::clone::Clone>(
    path: &str,
    start: &Option<T>,
    end: &Option<T>,
) -> Option<Document> {
    let mut range = doc! {};
    if let Some(start) = start {
        range.insert("gte", start);
    }
    if let Some(end) = end {
        range.insert("lte", end);
    }
    if range.is_empty() {
        return None;
    }
    range.insert("path", path);
    Some(doc! {"range": range})
}

pub fn query_range<T: Into<Bson> + std::clone::Clone>(
    start: &Option<T>,
    end: &Option<T>,
) -> Option<Document> {
    let mut range = doc! {};
    if let Some(start) = start {
        range.insert("$gte", start);
    }
    if let Some(end) = end {
        range.insert("$lte", end);
    }
    if range.is_empty() {
        return None;
    }
    Some(range)
}

pub fn init_logging() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.target(env_logger::Target::Stdout);
    builder.init();
}
