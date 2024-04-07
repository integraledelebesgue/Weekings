use std::{fs, io, collections::HashMap};
use actix_web::error::BlockingError;
use actix_web::web;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::database::badge::Badge;
use crate::database::group::Group;
use crate::database::quest::Quest;
use crate::database::user::User;
use crate::state::State;

async fn load(path: &str) -> Result<io::Result<Vec<u8>>, BlockingError> {
    let path = path.to_owned();
    web::block(move || fs::read(path)).await
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    id: String,
    name: String,
    users: HashMap<ObjectId, String>,
    categories: HashMap<ObjectId, String>,
    quests: HashMap<ObjectId, String>,
    posts: HashMap<ObjectId, String>,
}

impl Response {
    pub async fn from(state: Mutex<State>, group: ObjectId) -> mongodb::error::Result<Self> {
        let id = group.to_string();

        let repository = &state.lock().await.repository;

        let Group {
            name,
            users: users_ids,
            categories: categories_ids,
            quests: quests_ids,
            posts: posts_ids,
            ..
        } = repository
            .get_group(group)
            .await?;

        let mut users = HashMap::new();

        for id in users_ids {
            users.insert(id, repository.get_user(id).await?.name);
        }

        let mut categories = HashMap::new();

        for id in categories_ids {
            categories.insert(id, repository.get_category(id).await?.name);
        }

        let mut quests = HashMap::new();

        for id in quests_ids {
            let Quest { category, status, end_date, .. } = repository.get_quest(id).await?;
            let category = repository.get_category(category.unwrap()).await?.name;
            let repr = format!("{category} - {status} until {end_date}");

            quests.insert(id, repr);
        }

        let mut posts = HashMap::new();

        for id in posts_ids {
            posts.insert(id, repository.get_post(id).await?.text);
        }

        Ok(Response {
            id,
            name,
            users,
            categories,
            quests,
            posts
        })
    }
}
