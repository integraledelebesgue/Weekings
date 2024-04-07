use std::collections::HashMap;
use mongodb::bson::oid::ObjectId;
use crate::database::repository::Repository;

pub struct State {
    pub users: HashMap<String, ObjectId>,
    pub repository: Repository
}

impl State {
    pub fn new(repository: Repository) -> Self {
        let users = HashMap::<String, ObjectId>::new();
        State { users, repository }
    }
}

