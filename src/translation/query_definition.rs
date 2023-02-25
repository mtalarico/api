use std::{collections::HashMap};

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::util::date_wrapper::DateWrapper;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RangeType {
    #[serde(alias = "date")]
    Date(DateWrapper),
    #[serde(alias = "int")]
    Int(i32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RangeBound {
    #[serde(alias = "inclusive_upper", alias = "inclusiveUpper", alias = "]")]
    InclusiveUpper,
    #[serde(alias = "inclusive_lower", alias = "inclusiveLower", alias = "[")]
    InclusiveLower,
    #[serde(alias = "exclusive_upper", alias = "exclusiveUpper", alias = ")")]
    ExclusiveUpper,
    #[serde(alias = "exclusive_lower", alias = "exclusiveLower", alias = "(")]
    ExclusiveLower,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Range {
    bound: RangeBound,
    min: Option<RangeType>,
    max: Option<RangeType>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum QueryType {
    #[serde(alias = "equal", alias = "match")]
    Equal,
    #[serde(alias = "range")]
    Range(Range),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DataType {
    #[serde(alias = "string", alias = "str")]
    String,
    #[serde(alias = "object", alias = "oid")]
    ObjectId,
    #[serde(alias = "date", alias = "isodate", alias = "ISODate", alias = "datetime")]
    Date,
    #[serde(alias = "bool", alias = "boolean")]
    Bool,
    #[serde(alias = "int", alias = "i32", alias = "int32", alias = "integer")]
    Int,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mapping {
    field: String,
    datatype: DataType,
    query: QueryType,
    optional: Option<bool>
}

type FieldParameters = HashMap<APIField, Mapping>;
type APIField = String;
type CollectionName = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryDefinitions(HashMap<CollectionName, FieldParameters>);

impl QueryDefinitions {
    pub async fn parse_all_in_dir(dir_path: &str) -> Self {
        let mut definitions: Self = QueryDefinitions(HashMap::new());
        let mut file_paths = fs::read_dir(dir_path).await.unwrap();
        while let Some(file) = file_paths.next_entry().await.unwrap() {
            let path = file.path();
            let path = path.to_str().unwrap();
            if !file.file_type().await.unwrap().is_file() {
                log::info!("{} is not a file, skipping...", path);
                continue;
            }
            definitions = definitions.add_def(Self::parse_path(path).await);
        }
        definitions
    }


    async fn parse_path(path: &str) -> Self {
        let file = fs::read_to_string(path).await.expect("Unable to read file");
        let definitions = serde_json::from_str::<Self>(&file).unwrap();
        let names = definitions.get_names();
        log::info!("parsed translations for {:#?}", &names);
        log::debug!("{:#?}", &definitions);
        definitions
    }

    pub fn get_names(&self) -> Vec<String> {
        self.0.clone().into_keys().collect::<Vec<String>>()
    }

    fn add_def(&self, more: QueryDefinitions) -> Self {
        let mut copy = self.0.to_owned();
        copy.extend(more.0.to_owned());
        QueryDefinitions(copy)
    }
}
