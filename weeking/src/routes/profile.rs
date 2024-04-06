use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use crate::database::badge::Badge;

#[derive(Serialize, Deserialize)]
pub struct Response {
    user_id: String,
    name: String,
    photo: u32,  // placeholder
    friends: Vec<String>,
    groups: Vec<String>,
    badges: Vec<Badge>
}

impl Response {
    fn from(user: ObjectId) -> mongodb::error::Result<Self> {

    }
}
