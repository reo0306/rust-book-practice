use std::ops::Deref;

struct UserName(String);

impl Deref for UserName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct UserDeref {
    username: UserName,
}

impl UserDeref {
    pub fn new(username: String) -> Self { 
        Self { username: UserName(username) }
    }

    pub fn get_user_name(&self) -> &str {
        &self.username
    }
}
