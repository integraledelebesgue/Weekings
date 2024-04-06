use std::env;
use std::fmt::{Debug, Formatter};
use dotenv::dotenv;
use mongodb::{Client, Collection};
use derive_error::Error;
use mongodb::bson::oid::ObjectId;
use crate::database::post::Post;

use super::user::User;

#[derive(Debug, Error)]
pub enum InitError {
    InvalidUri,
    InvalidDatabase,
    ConnectionError
}

pub struct Repository {
    users: Collection<User>,
    // other collections
}

impl Debug for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Repository: Users")
    }
}

impl Repository {
    pub async fn init() -> Result<Self, InitError> {
        dotenv().ok();

        let uri = match env::var("DB_URI") {
            Ok(val) => Ok(val.to_string()),
            Err(_reason) => Err(InitError::InvalidUri)
        }?;

        let client = Client::with_uri_str(uri)
            .await.map_err(|_reason| InitError::ConnectionError)?;

        let name = match env::var("DB_NAME") {
            Ok(val) => Ok(val.to_string()),
            Err(_reason) => Err(InitError::InvalidUri)
        }?;

        let db = client.database(&name);

        let users: Collection<User> = db.collection("users");
        // other collections go here

        Ok(Repository { users })
    }

    pub async fn check_user_data(&self, name: &str, password: &str) -> mongodb::error::Result<()> {
        Ok(())
    }

    pub async fn get_user() -> Option<User> {
        todo!()
    }

    pub async fn add_user() -> mongodb::error::Result<()> {
        todo!()
    }

    pub async fn delete_user() -> mongodb::error::Result<()> {
        todo!()
    }

    pub async fn get_friends(user: ObjectId) -> Option<ObjectId> {
        todo!()
    }

    pub async fn make_friends() -> mongodb::error::Result<()> {
        todo!()
    }

    pub async fn delete_friends() -> mongodb::error::Result<()> {
        todo!()
    }

    pub async fn get_posts() -> mongodb::error::Result<()> {
        todo!()
    }

    pub async fn add_post() -> mongodb::error::Result<()> {
        todo!()
    }

    pub async fn delete_post() -> mongodb::error::Result<()> {
        todo!()
    }

    pub async fn post() -> mongodb::error::Result<()> {
        todo!()
    }


}