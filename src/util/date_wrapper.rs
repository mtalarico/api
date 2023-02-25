use mongodb::bson::{serde_helpers::bson_datetime_as_rfc3339_string, Bson, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct DateWrapper {
    #[serde(with = "bson_datetime_as_rfc3339_string")]
    datetime: DateTime,
}

impl Into<Bson> for DateWrapper {
    fn into(self) -> Bson {
        Bson::DateTime(self.datetime)
    }
}
