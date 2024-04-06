use std::env;
use std::fmt::{Debug, Formatter};
use dotenv::dotenv;
use mongodb::{Client, Collection};
use derive_error::Error;

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

    pub async fn add_user() {
        todo!()
    }

    pub async fn delete_user() {
        todo!()
    }

    pub async fn make_friends() {
        todo!()
    }

    pub async fn delete_friends() {
        todo!()
    }

    pub async fn post() {
        todo!()
    }

    pub async fn delete_post() {
        todo!()
    }


}