type ProductId = u32;
type ProductName = String;

pub struct Product {
    id: ProductId,
    name: ProductName,
    price: f64,
}

impl Product {
    pub fn new(id: u32, name: String, price: f64) -> Self {
        Self { id, name, price }
    }

    pub fn display(&self) {
        println!("Product ID: {}, Name: {}, Price: {}", &self.id, &self.name, &self.price);
    }
}