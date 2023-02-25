use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

use crate::{
    db,
    model::sequence::Sequence,
    util::{date_wrapper::DateWrapper, query_range},
    AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceQuery {
    pub airline_code: String,
    pub contract_month: String,
    pub seq_type: String,
    pub base: String,
    pub _id: Option<ObjectId>,
    pub base_indicator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_start: Option<DateWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date_end: Option<DateWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_start: Option<DateWrapper>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date_end: Option<DateWrapper>,
    pub duration_day_min: Option<i32>,
    pub duration_day_max: Option<i32>,
    pub num_legs_min: Option<i32>,
    pub num_legs_max: Option<i32>,
    pub odan: Option<bool>,
    pub red_eye: Option<bool>,
    pub training: Option<bool>,
    pub satellite: Option<bool>,
    pub observation_trip: Option<bool>,
}

pub async fn query(
    Query(payload): Query<SequenceQuery>,
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<Document>>) {
    log::info!("[/sequenceQuery] GET");
    log::debug!("[/sequenceQuery] recieved request {:?}", payload);
    let query = payload.to_query();

    let sequence_ns = mongodb::Namespace {
        db: "crew".to_string(),
        coll: "sequence".to_string(),
    };
    let start = Instant::now();
    let mut results = db::find(&state.db, &sequence_ns, query).await;
    let mut hits = vec![];
    while results.advance().await.unwrap() {
        let this: Document = results.deserialize_current().unwrap();
        log::debug!("[/sequenceQuery] found sequence {:?}", &this);
        hits.push(this);
    }
    let duration = start.elapsed();
    log::info!("[/sequenceQuery] took {:?} ms", duration);
    (StatusCode::OK, Json(hits))
}

impl SequenceQuery {
    pub fn to_query(&self) -> Document {
        let mut query = doc! {
            "airline_code": &self.airline_code,
            "contract_month": &self.contract_month,
            "seq_type": &self.seq_type,
            "base": &self.base,
        };

        if let Some(bi) = &self.base_indicator {
            query.insert("base_indicator", bi);
        }

        if let Some(id) = &self._id {
            query.insert("_id", id);
        }

        if let Some(start_range) = query_range(&self.start_date_start, &self.start_date_end) {
            query.insert("start_date", start_range);
        }
        if let Some(end_range) = query_range(&self.end_date_start, &self.end_date_end) {
            query.insert("end_date", end_range);
        }

        if let Some(duration_range) = query_range(&self.duration_day_min, &self.duration_day_max) {
            query.insert("duration_day", duration_range);
        }

        if let Some(leg_num_range) = query_range(&self.num_legs_min, &self.num_legs_max) {
            query.insert("num_legs", leg_num_range);
        }

        if let Some(odan) = &self.odan {
            query.insert("odan", odan);
        }

        if let Some(red_eye) = &self.red_eye {
            query.insert("red_eye", red_eye);
        }

        if let Some(training) = &self.training {
            query.insert("training", training);
        }

        if let Some(satellite) = &self.satellite {
            query.insert("satellite", satellite);
        }

        if let Some(ot) = &self.observation_trip {
            query.insert("observation_trip", ot);
        }
        log::debug!("made query {:?}", &query);
        query
    }
}
