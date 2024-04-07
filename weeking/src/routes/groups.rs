use std::collections::HashMap;
use actix_web::web::Data;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::database::user::User;
use crate::state::State;

#[derive(Serialize, Deserialize)]
pub struct Response {
    groups: HashMap<ObjectId, String>
}

impl Response {
    pub async fn from(state: Data<Mutex<State>>, user: ObjectId) -> mongodb::error::Result<Self> {
        let repository = &state.lock().await.repository;

        let User { groups: groups_id, .. } = repository
            .get_user(user)
            .await?;

        let mut groups =  HashMap::new();

        for id in repository.get_group_list(groups_id).await?.groups {
            groups.insert(id, repository.get_group(id).await?.name);
        }

        Ok(Response { groups })
    }
}