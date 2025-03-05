type UserId = i32;
type UserName = String;

pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }

    pub fn display(&self) {
        println!("User ID: {}, Nmae: {}", &self.id, &self.name);
    }
}