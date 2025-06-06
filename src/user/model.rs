#[derive(Clone, Debug)]
pub struct User {
    pub user_id: UserId,
    pub username: Username,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserDraft {
    pub username: Username,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct UserId(pub u32);
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Username(pub String);
