use std::marker::PhantomData;

pub struct UserId;
pub struct UserName;
pub struct ProductId;
pub struct ProductName;

pub struct Id<T> {
    id: i32,
    _phantom: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: i32) -> Self {
        Self { id, _phantom: PhantomData }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }
}

pub struct Name<T> {
    name: String,
    _phantom: PhantomData<T>,
}

impl<T> Name<T> {
    pub fn new(name: String) -> Self {
        Self { name, _phantom: PhantomData }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

pub struct PhantomUser {
    pub id: Id<UserId>,
    pub name: Name<UserName>,
}

impl PhantomUser {
    pub fn new(id: Id<UserId>, name: Name<UserName>) -> Self {
        Self {
            id, name
        }
    }
}

pub struct PhantomProduct {
    pub id: Id<ProductId>,
    pub name: Name<ProductName>,
}

impl PhantomProduct {
    pub fn new(id: Id<ProductId>, name: Name<ProductName>) -> Self {
        Self {
            id, name
        }
    }
}