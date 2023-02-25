use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sequence {
    _id: ObjectId,
    airline_code: String,
    contract_month: String,
    seq_type: String,
    base: String,
    observation_trip: bool,
    base_indicator: String,
    start_date: DateTime,
    end_date: DateTime,
    duration_day: i32,
    num_legs: i32,
    odan: bool,
    red_eye: bool,
    training: bool,
    satellite: bool,
}
