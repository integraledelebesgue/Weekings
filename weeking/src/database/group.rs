use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Group {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub users: Vec<ObjectId>,
    pub categories: Vec<ObjectId>,
    pub quests: Vec<ObjectId>,
    pub posts: Vec<ObjectId>,
}