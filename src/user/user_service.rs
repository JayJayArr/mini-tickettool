use std::collections::HashMap;

use super::model::{User, UserDraft, UserId};

trait UserRepository {
    async fn get_users(&self) -> Vec<User>;
    async fn get_user_by_id(&self, user_id: &UserId) -> Option<&User>;
    async fn getmut_user_by_id(&mut self, user_id: &UserId) -> Option<&mut User>;
    async fn create_user(&mut self, create_user: UserDraft) -> User;
    async fn delete_user(&mut self, user_id: &UserId) -> Option<User>;
}

pub struct InMemUserRepository {
    users: HashMap<UserId, User>,
    counter: u32,
}

impl InMemUserRepository {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            counter: 1,
        }
    }
}

impl UserRepository for InMemUserRepository {
    async fn get_users(&self) -> Vec<User> {
        self.users.values().cloned().collect()
    }
    async fn get_user_by_id(&self, user_id: &UserId) -> Option<&User> {
        self.users.get(user_id)
    }
    async fn getmut_user_by_id(&mut self, user_id: &UserId) -> Option<&mut User> {
        self.users.get_mut(user_id)
    }
    async fn create_user(&mut self, create_user: UserDraft) -> User {
        self.counter += 1;
        let user = User {
            user_id: UserId(self.counter),
            username: create_user.username,
        };
        self.users.insert(UserId(self.counter), user).unwrap()
    }
    async fn delete_user(&mut self, user_id: &UserId) -> Option<User> {
        self.users.remove(user_id)
    }
}
