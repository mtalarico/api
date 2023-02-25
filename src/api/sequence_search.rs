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
    util::{date_wrapper::DateWrapper, search_range},
    AppState,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SequenceSearch {
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

pub async fn search(
    Query(payload): Query<SequenceSearch>,
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<Sequence>>) {
    log::info!("[/sequenceSearch] GET");
    log::debug!("[/sequenceSearch] recieved request {:?}", payload);
    let pipeline = payload.to_search_pipeline();

    let sequence_ns = mongodb::Namespace {
        db: "crew".to_string(),
        coll: "sequence".to_string(),
    };
    let start = Instant::now();
    let mut results = db::aggregate(&state.db, &sequence_ns, pipeline).await;
    let mut hits = vec![];
    while results.advance().await.unwrap() {
        let this: Sequence =
            mongodb::bson::from_document(results.deserialize_current().unwrap()).unwrap();
        log::debug!("[/sequenceSearch] found sequence {:?}", &this);
        hits.push(this);
    }
    let duration = start.elapsed();
    log::info!("[/sequenceSearch] took {:?} ms", duration);
    (StatusCode::OK, Json(hits))
}

impl SequenceSearch {
    pub fn to_search_pipeline(&self) -> Vec<Document> {
        let mut musts = vec![
            doc! {"text": { "path": "airline_code", "query": &self.airline_code }},
            doc! {"text": { "path": "contract_month", "query": &self.contract_month }},
            doc! {"text": { "path": "seq_type", "query": &self.seq_type }},
            doc! {"text": { "path": "base", "query": &self.base }},
        ];

        if let Some(bi) = &self.base_indicator {
            musts.push(doc! {"text": { "path": "base_indicator", "query": bi }});
        }

        if let Some(id) = &self._id {
            musts.push(doc! {"equals": { "path": "_id", "value": id}})
        }

        if let Some(start_range) =
            search_range("start_date", &self.start_date_start, &self.start_date_end)
        {
            musts.push(start_range);
        }
        if let Some(end_range) = search_range("end_date", &self.end_date_start, &self.end_date_end)
        {
            musts.push(end_range);
        }

        if let Some(duration_range) = search_range(
            "duration_day",
            &self.duration_day_min,
            &self.duration_day_max,
        ) {
            musts.push(duration_range);
        }

        if let Some(leg_num_range) =
            search_range("num_legs", &self.num_legs_min, &self.num_legs_max)
        {
            musts.push(leg_num_range);
        }

        if let Some(odan) = &self.odan {
            musts.push(doc! {"equals": { "path": "odan", "value": odan }});
        }

        if let Some(red_eye) = &self.red_eye {
            musts.push(doc! {"equals": { "path": "red_eye", "value": red_eye }});
        }

        if let Some(training) = &self.training {
            musts.push(doc! {"equals": { "path": "training", "value": training }});
        }

        if let Some(satellite) = &self.satellite {
            musts.push(doc! {"equals": { "path": "satellite", "value": satellite }});
        }

        if let Some(ot) = &self.observation_trip {
            musts.push(doc! {"equals": { "path": "observation_trip", "value": ot }});
        }

        let pipeline = vec![doc! {
            "$search": {
                "index": "default",
                "compound": {
                    "must": musts
                }
            }
        }];
        log::debug!("made pipeline {:?}", &pipeline);
        pipeline
    }
}
