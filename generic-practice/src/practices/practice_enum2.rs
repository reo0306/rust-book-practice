pub enum PaymentMethod {
    CreditCard,
    Paypal,
    Bitcoin,
}

pub struct PaymentEx {
    method: PaymentMethod,
    amount: f64,
    identifier: String,
}

impl PaymentEx {
    pub fn new(method: PaymentMethod, amount: f64, identifier: String) -> Self {
        Self {
            method,
            amount,
            identifier,
        }
    }

    pub fn process_payment(&self) {
        match &self.method {
            PaymentMethod::CreditCard => println!("Processing Credit Card payment of {} using card: {}", &self.amount, &self.identifier),
            PaymentMethod::Paypal => println!("Processing PayPal payment of {} using email: {}", &self.amount, &self.identifier),
            PaymentMethod::Bitcoin => println!("Processing Bitcoin payment of {} using wallet: {}", &self.amount, &self.identifier),
        };
    }
}