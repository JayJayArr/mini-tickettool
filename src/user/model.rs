#[derive(Clone)]
pub struct User {
    pub user_id: UserId,
    pub username: Username,
}

pub struct UserDraft {
    pub username: Username,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub u32);
#[derive(Clone)]
pub struct Username(String);
