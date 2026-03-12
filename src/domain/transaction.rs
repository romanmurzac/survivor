//! This module provides a unified interface for all financial events.
//!
//! By wrapping specific types into the `Transaction` enum, the simulation engine
//! can process diverse financial entities (Incomes, Expenses, Loans) using
//! a consistent API.

use crate::domain::scenario::Target;
use crate::domain::{expense::Expense, income::Income, loan::Loan};

/// A polymorphic container for any financial event that affects cash flow.
#[derive(Debug)]
pub enum Transaction {
    /// Represents a financial inflow.
    Income(Income),
    /// Represents a financial outflow.
    Expense(Expense),
    /// Represents a debt repayment.
    Loan(Loan),
}

impl Transaction {
    /// Determines the reporting `Target` for the underlying transaction.
    ///
    /// Note: `Loan` transactions are currently categorized as `Target::FixExpense`.
    pub fn target(&self) -> Target {
        match self {
            Transaction::Income(inc) => inc.target(),
            Transaction::Expense(exp) => exp.target(),
            Transaction::Loan(_) => Target::FixExpense,
        }
    }

    /// Calculates the monetary impact of the transaction for a given month.
    ///
    /// For stateful transactions like `Loan`, this call will modify the internal
    /// balance of the loan.
    ///
    /// # Arguments
    /// * `month` - The current simulation month index.
    pub fn process(&mut self, month: u32) -> f64 {
        match self {
            Transaction::Income(inc) => inc.process(month),
            Transaction::Expense(exp) => exp.process(month),
            Transaction::Loan(loan) => loan.process(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::expense::{Expense, ExpenseType};
    use crate::domain::income::{Income, IncomeCategory, IncomeType};
    use crate::domain::loan::Loan;

    #[test]
    fn test_transaction_delegation() {
        let mut inc = Transaction::Income(Income::new(
            "Salary",
            100.0,
            IncomeType::Recurrent,
            IncomeCategory::Active,
        ));
        let mut exp = Transaction::Expense(Expense::new("Rent", 50.0, ExpenseType::Fix));

        // Month 1.
        assert_eq!(inc.process(1), 100.0);
        assert_eq!(exp.process(1), 50.0);
    }

    #[test]
    fn test_loan_target_mapping() {
        let loan = Transaction::Loan(Loan::new("Bank", 1000.0, 100.0, 10.0));
        // Verify that loans currently map to FixExpense target.
        assert_eq!(loan.target(), Target::FixExpense);
    }
}
