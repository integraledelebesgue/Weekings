use std::fmt::{Display, Formatter};
use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug,  Serialize, Deserialize)]
pub enum Status {
    Completed,
    Failed,
    InProgress
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Quest {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub status: Status,
    pub start_date: DateTime,
    pub end_date: DateTime,
    pub complete_date: DateTime,
    pub category: Option<ObjectId>
}