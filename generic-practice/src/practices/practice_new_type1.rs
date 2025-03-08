struct UserId(i32);

impl UserId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

pub struct UserEx {
    id: UserId,
    name: String,
}

impl UserEx {
    pub fn new(id: i32, name: String) -> Self {
        Self { id: UserId(id), name }
    }

    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
}