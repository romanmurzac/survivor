//! This module handles loan debt tracking and repayment logic.
//!
//! Loans maintain an internal state (the remaining balance)
//! which decreases over time as payments are processed.

/// Represents a financial debt with a fixed repayment structure.
#[derive(Clone, Debug)]
pub struct Loan {
    /// The name of the loan.
    pub name: String,
    /// The current outstanding balance of the loan.
    pub remaining: f64,
    /// The portion of the monthly payment that reduces the balance.
    pub principal: f64,
    /// The portion of the monthly payment that covers interest.
    pub interest: f64,
}

impl Loan {
    /// Creates a new `Loan` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - Description of the debt.
    /// * `remaining` - The total starting balance.
    /// * `principal` - The fixed monthly amount applied to the debt.
    /// * `interest` - The fixed monthly interest charge.
    pub fn new(name: impl Into<String>, remaining: f64, principal: f64, interest: f64) -> Self {
        Self {
            name: name.into(),
            remaining,
            principal,
            interest,
        }
    }

    /// Processes a single payment cycle, reducing the remaining balance and returning the total cost.
    ///
    /// The payment returned is the sum of `principal` and `interest`. If the `remaining` balance
    /// hits zero, the loan stops generating payments.
    ///
    /// # Returns
    ///
    /// Returns the total payment amount ($principal + interest$) for the current month.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::domain::loan::Loan;
    /// let mut loan = Loan::new("Small Loan", 100.0, 50.0, 10.0);
    ///
    /// // Month 1: 50 principal + 10 interest = 60
    /// assert_eq!(loan.process(), 60.0);
    /// assert_eq!(loan.remaining, 50.0);
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loan_lifecycle() {
        // Loan of 100, paying 40 principal + 5 interest per month
        let mut loan = Loan::new("Test", 100.0, 40.0, 5.0);

        // Month 1
        assert_eq!(loan.process(), 45.0);
        assert_eq!(loan.remaining, 60.0);

        // Month 2
        assert_eq!(loan.process(), 45.0);
        assert_eq!(loan.remaining, 20.0);

        // Month 3 (Principal exceeds remaining)
        assert_eq!(loan.process(), 45.0);
        assert_eq!(loan.remaining, 0.0);

        // Month 4 (Loan is finished)
        assert_eq!(loan.process(), 0.0);
    }

    #[test]
    fn test_zero_balance_immediately_returns_zero() {
        let mut loan = Loan::new("Paid", 0.0, 100.0, 10.0);
        assert_eq!(loan.process(), 0.0);
    }
}
