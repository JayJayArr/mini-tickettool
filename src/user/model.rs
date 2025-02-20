pub struct User {
    user_id: UserId,
    username: Username,
}

pub struct UserDraft {
    pub username: Username,
}
pub struct UserId(u32);
pub struct Username(String);
