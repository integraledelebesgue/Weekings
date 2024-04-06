use mongodb::bson::DateTime;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub creation_date: DateTime,
    pub text: String,
    pub category: Option<ObjectId>,
    pub owner: Option<ObjectId>
}