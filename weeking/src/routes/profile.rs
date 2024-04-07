use std::collections::HashMap;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::database::badge::Badge;
use crate::database::user::User;
use crate::state::State;

#[derive(Serialize, Deserialize)]
pub struct Response {
    id: String,
    name: String,
    friends: HashMap<ObjectId, String>,
    groups: HashMap<ObjectId, String>,
    badges: HashMap<ObjectId, String>
}

impl Response {
    pub async fn from(state: Mutex<State>, user: ObjectId) -> mongodb::error::Result<Self> {
        let id = user.to_string();

        let repository = &state.lock().await.repository;

        let User {
            name,
            friends: friends_id,
            groups: groups_id,
            badges: badges_id,
            ..
        } = repository
            .get_user(user)
            .await?;

        let mut friends = HashMap::new();

        for id in repository.get_user_list(friends_id).await?.users {
            friends.insert(id, repository.get_user(id).await?.name);
        }

        let mut groups = HashMap::new();

        for id in repository.get_group_list(groups_id).await?.groups {
            groups.insert(id, repository.get_group(id).await?.name);
        }

        let mut badges = HashMap::new();

        for id in repository.get_badge_list(badges_id).await?.badges {
            badges.insert(id, repository.get_badge(id).await?.name);
        }

        Ok(Response {
            id,
            name,
            friends,
            groups,
            badges
        })
    }
}
