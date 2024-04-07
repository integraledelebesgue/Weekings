use std::env;
use std::fmt::{Debug, Formatter};
use dotenv::dotenv;
use mongodb::{Client, Collection};
use derive_error::Error;
use mongodb::bson::{DateTime, doc};
use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOneOptions;
use mongodb::results::DeleteResult;
use crate::database::badge::Badge;
use crate::database::category::Category;
use crate::database::group::Group;
use crate::database::post::Post;
use crate::database::quest::Quest;
use crate::database::badge_list::BadgeList;
use crate::database::category_list::CategoryList;
use crate::database::group_list::GroupList;
use crate::database::post_list::PostList;
use crate::database::quest_list::QuestList;
use crate::database::user_list::UserList;

use super::user::User;

#[derive(Debug, Error)]
pub enum InitError {
    InvalidUri,
    InvalidDatabase,
    ConnectionError
}

pub struct Repository {
    badges: Collection<Badge>,
    badge_lists: Collection<BadgeList>,
    categories: Collection<Category>,
    category_lists: Collection<CategoryList>,
    groups: Collection<Group>,
    group_lists: Collection<GroupList>,
    posts: Collection<Post>,
    post_lists: Collection<PostList>,
    quests: Collection<Quest>,
    quest_lists: Collection<QuestList>,
    users: Collection<User>,
    user_lists: Collection<UserList>
}

impl Debug for Repository {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Repository: Users")
    }
}

const DB_URI: &'static str = "mongodb+srv://rust-app:weekings2137#$@weekings.jpw1wfw.mongodb.net/?retryWrites=true&w=majority&appName=Weekings";
const DB_NAME: &'static str = "Weekings";


impl Repository {
    pub async fn init() -> Result<Self, InitError> {
        dotenv().ok();

        // let uri = match env::var("DB_URI") {
        //     Ok(val) => Ok(val.to_string()),
        //     Err(_reason) => Err(InitError::InvalidUri)
        // }?;

        let client = Client::with_uri_str(DB_URI)
            .await.map_err(|_reason| InitError::ConnectionError)?;

        // let name = match env::var("DB_NAME") {
        //     Ok(val) => Ok(val.to_string()),
        //     Err(_reason) => Err(InitError::InvalidUri)
        // }?;

        let db = client.database(DB_NAME);

        let badges: Collection<Badge> = db.collection("Badges");
        let badge_lists: Collection<BadgeList> = db.collection("BadgeLists");

        let categories: Collection<Category> = db.collection("Badges");
        let category_lists: Collection<CategoryList> = db.collection("BadgeLists");

        let groups: Collection<Group> = db.collection("Groups");
        let group_lists: Collection<GroupList> = db.collection("GroupLists");

        let posts: Collection<Post> = db.collection("Posts");
        let post_lists: Collection<PostList> = db.collection("PostLists");

        let quests: Collection<Quest> = db.collection("Quests");
        let quest_lists: Collection<QuestList> = db.collection("QuestLists");

        let users: Collection<User> = db.collection("Users");
        let user_lists: Collection<UserList> = db.collection("UserLists");

        Ok(Repository {
            badges,
            badge_lists,
            categories,
            category_lists,
            groups,
            group_lists,
            posts,
            post_lists,
            quests,
            quest_lists,
            users,
            user_lists
        })
    }

    async fn create_user_list(&mut self) -> mongodb::error::Result<ObjectId> {
        let id = ObjectId::new();
        let users = Vec::<ObjectId>::new();

        let list = UserList { id: Some(id), users };

        self.user_lists
            .insert_one(&list, None)
            .await?;

        return Ok(id);
    }

    async fn create_group_list(&mut self) -> mongodb::error::Result<ObjectId> {
        let id = ObjectId::new();
        let groups = Vec::<ObjectId>::new();

        let list = GroupList { id: Some(id), groups };

        self.group_lists
            .insert_one(&list, None)
            .await?;

        return Ok(id);
    }

    async fn create_badge_list(&mut self) -> mongodb::error::Result<ObjectId> {
        let id = ObjectId::new();
        let badges = Vec::<ObjectId>::new();

        let list = BadgeList { id: Some(id), badges };

        self.badge_lists
            .insert_one(&list, None)
            .await?;

        return Ok(id);
    }

    pub async fn get_user_list(&self, id: ObjectId) -> mongodb::error::Result<UserList> {
        Ok(self.user_lists.find_one(doc! { "_id": id }, None).await?.unwrap())
    }

    pub async fn get_group_list(&self, id: ObjectId) -> mongodb::error::Result<GroupList> {
        Ok(self.group_lists.find_one(doc! { "_id": id }, None).await?.unwrap())
    }

    pub async fn get_badge_list(&self, id: ObjectId) -> mongodb::error::Result<BadgeList> {
        Ok(self.badge_lists.find_one(doc! { "_id": id }, None).await?.unwrap())
    }

    async fn create_quest_list(&mut self) -> mongodb::error::Result<ObjectId> {
        let id = ObjectId::new();
        let quests = Vec::<ObjectId>::new();

        let list = QuestList { id: Some(id), quests };

        self.quest_lists
            .insert_one(&list, None)
            .await?;

        return Ok(id);
    }

    async fn create_post_list(&mut self) -> mongodb::error::Result<ObjectId> {
        let id = ObjectId::new();
        let posts = Vec::<ObjectId>::new();

        let list = PostList { id: Some(id), posts };

        self.post_lists
            .insert_one(&list, None)
            .await?;

        return Ok(id);
    }

    pub async fn check_user_data(&self, username: &str, password: &str) -> mongodb::error::Result<Option<ObjectId>> {
        let query = doc! {
            "username": username,
            "password": password
        };

        let user = self.users
            .find_one(query, None)
            .await?;

        Ok(user.map(|u| u.id.unwrap()))
    }

    pub async fn get_user(&self, id: ObjectId) -> mongodb::error::Result<User> {
        Ok(
            self.users.find_one(
                doc! { "_id": id },
                None
            )
                .await?
                .unwrap()
        )
    }

    pub async fn add_user(
        &mut self,
        name: String,
        username: String,
        mail: String,
        password: String,
    ) -> mongodb::error::Result<ObjectId> {
        let id = ObjectId::new();

        let friends = self.create_user_list().await?;
        let groups = self.create_group_list().await?;
        let badges = self.create_badge_list().await?;
        let quests = self.create_quest_list().await?;
        let inspirations = self.create_post_list().await?;
        let posts = self.create_post_list().await?;

        let user = User {
            id: Some(id),
            name,
            username,
            mail,
            password,
            friends,
            groups,
            badges,
            quests,
            inspirations,
            posts
        };

        self.users.insert_one(&user, None).await?;

        Ok(id)
    }

    pub async fn delete_user(&mut self, id: ObjectId) -> mongodb::error::Result<DeleteResult> {
        self.users
            .delete_one(doc! { "_id": id }, None)
            .await
    }

    // pub async fn make_friends(&mut self, user_id1: ObjectId, user_id2: ObjectId) -> mongodb::error::Result<()> {
    //     let user1 = self
    //         .get_user(user_id1)
    //         .await?;
    //
    //     let user2 = self
    //         .get_user(user_id2)
    //         .await?;
    //
    //     let filter1 = doc! { "_id": &user1.friends };
    //     let filter2 = doc! { "_id": &user2.friends };
    //
    //     let mut friends1 = self
    //         .user_lists
    //         .find_one(filter1.clone(), None)
    //         .await?
    //         .unwrap();
    //
    //     let mut friends2 = self
    //         .user_lists
    //         .find_one(filter2.clone(), None)
    //         .await?
    //         .unwrap();
    //
    //     friends2.users.push(user_id1);
    //     friends1.users.push(user_id2);
    //
    //     self.user_lists.update_one(filter1, doc! { "users": bson::to_bson(&friends1).unwrap() }, None).await?;
    //     self.user_lists.update_one(filter2, doc! { "users": bson::to_bson(&friends2) }, None).await?;
    //
    //     Ok(())
    // }
    //
    // pub async fn delete_friends(&mut self, user_id1: ObjectId, user_id2: ObjectId) -> mongodb::error::Result<()> {
    //     let user1 = self
    //         .get_user(user_id1)
    //         .await?;
    //
    //     let user2 = self
    //         .get_user(user_id2)
    //         .await?;
    //
    //     let filter1 = doc! { "_id": &user1.friends };
    //     let filter2 = doc! { "_id": &user2.friends };
    //
    //     let UserList { id, users: mut friends } = self
    //         .user_lists
    //         .find_one(filter1.clone(), None)
    //         .await?
    //         .unwrap();
    //
    //     friends.retain(|u| *u != user_id2);
    //
    //     let friends1 = UserList { id, users: friends };
    //
    //     let UserList { id, users: mut friends } = self
    //         .user_lists
    //         .find_one(filter2.clone(), None)
    //         .await?
    //         .unwrap();
    //
    //     friends.retain(|u| *u != user_id1);
    //
    //     let friends2 = UserList { id, users: friends };
    //
    //     self.user_lists.update_one(filter1, doc! { "users": bson::to_bson(&friends1) }, None).await?;
    //     self.user_lists.update_one(filter2, doc! { "users": bson::to_bson(&friends2) }, None).await?;
    //
    //     Ok(())
    // }

    pub async fn get_post(&self, id: ObjectId) -> mongodb::error::Result<Post> {
        Ok(self.posts.find_one(doc! { "_id": id }, None).await?.unwrap())
    }

    pub async fn add_post(
        &mut self,
        user: ObjectId,
        creation_date: DateTime,
        text: String,
        photo_path: String,
        category: ObjectId
    ) -> mongodb::error::Result<ObjectId> {
        let id = ObjectId::new();

        let post = Post {
            id: Some(id),
            user,
            creation_date,
            text,
            photo_path,
            category
        };

        self.posts.insert_one(&post, None).await?;

        let user_id = user;

        let user = self
            .users
            .find_one(doc! { "_id": user_id }, None)
            .await?
            .unwrap();

        let posts_id = user.posts;
        let filter = doc! { "_id": posts_id };

        let mut posts = self
            .post_lists
            .find_one(filter.clone(), None)
            .await?
            .unwrap();

        posts.posts.push(id);

        self.posts
            .update_one(filter, doc! { "posts": bson::to_bson(&posts).unwrap() }, None)
            .await?;

        Ok(id)
    }

    pub async fn delete_post(&mut self, id: ObjectId) -> mongodb::error::Result<DeleteResult> {
        self.posts
            .delete_one(doc! { "_id": id }, None)
            .await
    }

    pub async fn get_group_by_name(&self, name: &str) -> mongodb::error::Result<Option<Group>> {
        Ok(self.groups.find_one(doc! { "name": name }, None).await?)
    }

    pub async fn get_group(&self, id: ObjectId) -> mongodb::error::Result<Group> {
        Ok(self.groups.find_one(doc! { "_id": id }, None).await?.unwrap())
    }

    pub async fn add_group(&mut self) -> mongodb::error::Result<ObjectId> {
        todo!()
    }

    pub async fn get_quest(&self, id: ObjectId) -> mongodb::error::Result<Quest> {
        Ok(self.quests.find_one(doc! { "_id": id }, None).await?.unwrap())
    }

    pub async fn add_quest(&mut self) -> mongodb::error::Result<ObjectId> {
        todo!()
    }

    pub async fn complete_quest(&mut self) {

    }

    pub async fn get_category_by_name(&self, name: &str) -> mongodb::error::Result<Option<Category>> {
        Ok(self.categories.find_one(doc! { "name": name }, None).await?)
    }

    pub async fn get_category(&self, id: ObjectId) -> mongodb::error::Result<Category> {
        Ok(self.categories.find_one(doc! { "_id": id }, None).await?.unwrap())
    }

    pub async fn get_badge(&self, id: ObjectId) -> mongodb::error::Result<Badge> {
        todo!()
    }

    pub async fn grant_badge(&mut self) {

    }

}