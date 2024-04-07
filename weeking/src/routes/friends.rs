use std::collections::HashMap;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::database::user::User;
use crate::state::State;

#[derive(Serialize, Deserialize)]
pub struct Response {
    friends: HashMap<ObjectId, String>
}

impl Response {
    pub async fn from(state: Mutex<State>, user: ObjectId) -> mongodb::error::Result<Self> {
        let repository = &state.lock().await.repository;

        let User { friends: id, .. } = repository
            .get_user(user)
            .await?;

        let mut friends = HashMap::new();

        for id in repository.get_user_list(id).await?.users {
            friends.insert(id, repository.get_user(id).await?.name);
        }

        Ok(Response { friends })
    }
}