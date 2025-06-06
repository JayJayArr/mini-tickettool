use std::collections::HashMap;

use super::model::{User, UserDraft, UserId};

trait UserRepository {
    fn get_users(&self) -> Vec<User>;
    fn get_user_by_id(&self, user_id: &UserId) -> Option<&User>;
    fn getmut_user_by_id(&mut self, user_id: &UserId) -> Option<&mut User>;
    fn create_user(&mut self, create_user: UserDraft) -> UserId;
    fn delete_user(&mut self, user_id: &UserId) -> Option<User>;
    fn counter(&mut self) -> u32;
}

pub struct InMemUserRepository {
    users: HashMap<UserId, User>,
    counter: u32,
}

impl InMemUserRepository {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            counter: 0,
        }
    }
}

impl UserRepository for InMemUserRepository {
    fn get_users(&self) -> Vec<User> {
        self.users.values().cloned().collect()
    }
    fn get_user_by_id(&self, user_id: &UserId) -> Option<&User> {
        self.users.get(user_id)
    }
    fn getmut_user_by_id(&mut self, user_id: &UserId) -> Option<&mut User> {
        self.users.get_mut(user_id)
    }
    fn create_user(&mut self, create_user: UserDraft) -> UserId {
        self.counter += 1;
        let id = UserId(self.counter);
        let user = User {
            user_id: UserId(self.counter),
            username: create_user.username,
        };
        self.users.insert(UserId(self.counter), user);
        id
    }
    fn delete_user(&mut self, user_id: &UserId) -> Option<User> {
        self.users.remove(user_id)
    }
    fn counter(&mut self) -> u32 {
        self.counter
    }
}

#[cfg(test)]
mod tests {
    use crate::user::{
        model::{UserDraft, UserId, Username},
        user_service::UserRepository,
    };

    use super::InMemUserRepository;

    #[test]
    fn create_user() {
        let mut repo = InMemUserRepository::new();

        let user = UserDraft {
            username: Username("Jakob".to_string()),
        };
        assert_eq!(repo.counter(), 0);
        let userid = repo.create_user(user);
        println!("{:?}", userid);
        assert_eq!(repo.counter(), 1);
    }

    #[test]
    fn get_non_existing_user() {
        let repo = InMemUserRepository::new();
        let result = repo.get_user_by_id(&UserId(1));
        assert!(result.is_none())
    }

    #[test]
    fn delete_user() {
        let mut repo = InMemUserRepository::new();
        let user = UserDraft {
            username: Username("Jakob".to_string()),
        };
        let userid = repo.create_user(user.clone());
        let deleted_user = repo.delete_user(&userid).unwrap();
        assert_eq!(deleted_user.username, user.username);
        assert_eq!(deleted_user.user_id.0, 1);
        let result = repo.get_user_by_id(&userid);
        assert!(result.is_none())
    }

    #[test]
    fn get_user() {
        let mut repo = InMemUserRepository::new();
        let user = UserDraft {
            username: Username("Jakob".to_string()),
        };
        let userid = repo.create_user(user.clone());
        let user = repo.get_user_by_id(&userid);
        assert!(user.is_some());
    }

    #[test]
    fn get_user_mut() {
        let mut repo = InMemUserRepository::new();
        let user = UserDraft {
            username: Username("Jakob".to_string()),
        };
        let userid = repo.create_user(user);
        let user = repo.getmut_user_by_id(&userid);
        assert!(user.is_some());
        let new_username = Username("Jakobchanged".to_string());
        user.unwrap().username = new_username.clone();
        assert_eq!(repo.get_user_by_id(&userid).unwrap().username, new_username);
    }

    #[test]
    fn get_users() {
        let mut repo = InMemUserRepository::new();
        let user1 = UserDraft {
            username: Username("Jakob".to_string()),
        };
        let user2 = UserDraft {
            username: Username("Rob".to_string()),
        };
        let _ = repo.create_user(user1);
        let _ = repo.create_user(user2);
        assert_eq!(repo.get_users().len(), 2);
    }
}
