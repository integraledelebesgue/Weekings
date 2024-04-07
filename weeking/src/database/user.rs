use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub username: String,  // to login only
    pub mail: String,
    pub password: String,
    pub friends: ObjectId,  // ref to object in other collection
    pub groups: ObjectId,
    pub badges: ObjectId,
    pub quests: ObjectId,
    pub inspirations: ObjectId,
    pub posts: ObjectId
}