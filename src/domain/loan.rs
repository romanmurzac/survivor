#[derive(Clone, Debug)]
pub struct Loan {
    pub name: String,
    pub remaining: f64,
    pub principal: f64,
    pub interest: f64,
}

impl Loan {
    pub fn new(name: impl Into<String>, remaining: f64, principal: f64, interest: f64) -> Self {
        Self {
            name: name.into(),
            remaining,
            principal,
            interest,
        }
    }

    pub fn process(&mut self) -> f64 {
        if self.remaining <= 0.0 {
            return 0.0;
        }

        let payment = self.principal + self.interest;
        
        self.remaining -= self.principal;

        if self.remaining < 0.0 {
            self.remaining = 0.0;
        }

        payment
    }
}
