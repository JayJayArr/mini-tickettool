use std::collections::HashMap;

use super::model::{User, UserDraft, UserId};

trait UserRepository {
    async fn get_userss(&self) -> Vec<User>;
    async fn get_user_by_id(&self, ticket_id: &UserId) -> Option<&User>;
    async fn getmut_user_by_id(&mut self, ticket_id: &UserId) -> Option<&mut User>;
    async fn create_user(&mut self, create_ticket: UserDraft) -> User;
    async fn delete_user(&mut self, ticket_id: &UserId) -> Option<User>;
}

struct InMemUserRepository {
    users: HashMap<UserId, User>,
    counter: u32,
}

impl InMemUserRepository {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            counter: 1,
        }
    }
}
