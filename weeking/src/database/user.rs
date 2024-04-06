use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub username: String,  // to login only
    pub mail: String,
    pub password: String,
    pub friends: Option<ObjectId>,  // ref to object in other collection
    pub groups: Option<ObjectId>,
    pub badges: Option<ObjectId>,
    pub quests: Option<ObjectId>,
    pub inspirations: Option<ObjectId>,
    pub posts: Option<ObjectId>
}