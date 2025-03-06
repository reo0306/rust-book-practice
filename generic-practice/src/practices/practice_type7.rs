type Amount = f64;

pub trait Payment {
    fn process_payment(&self, amount: Amount);
}

pub struct CreditCardPayment;

impl CreditCardPayment {
    pub fn new() -> Self {
        Self
    }
}

impl Payment for CreditCardPayment {
    fn process_payment(&self, amount: f64) {
        println!("Processing credit card payment: {}", amount);
    }
}

pub struct PaypalPayment;

impl PaypalPayment {
    pub fn new() -> Self {
        Self
    }
}

impl Payment for PaypalPayment {
    fn process_payment(&self, amount: f64) {
        println!("Processing Paypal payment: {}", amount);
    }
}