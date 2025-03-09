use std::ops::Deref;

struct UserId(i32);

impl Deref for UserId {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct UserName(String);

impl Deref for UserName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub struct UserDerefEx {
    id: UserId,
    name: UserName,
}

impl UserDerefEx {
    pub fn new(id: i32, name: String) -> Self {
        Self { id: UserId(id), name: UserName(name) }
    }

    pub fn get_id(&self) -> i32 {
        *self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}