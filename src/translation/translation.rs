use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio::fs;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RangeBound {
    InclusiveUpper,
    InclusiveLower,
    ExclusiveUpper,
    ExclusiveLower,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QueryType {
    StringMatch,
    BoolMatch,
    ObjectIDMatch,
    DateRange(RangeBound),
    Int32Range(RangeBound),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mapping {
    field: String,
    query_type: QueryType,
}

type Definitions = HashMap<String, Mapping>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Translations(HashMap<String, Definitions>);

impl Translations {
    pub async fn parse_all_in_dir(dir_path: &str) -> Self {
        let file = fs::read_to_string(dir_path).await.expect("Unable to read file");
        let translation: Self = serde_json::from_str::<Self>(&file).unwrap();
        let names = translation.get_names();
        log::debug!("parsed translations for {} at {:?}", &names, &translation);
        translations
    }

    pub fn get_names(&self) -> Vec<String> {
        self.
    }
}
