use crate::domain::scenario::Target;
use crate::domain::{expense::Expense, income::Income, loan::Loan};

#[derive(Debug)]
pub enum Transaction {
    Income(Income),
    Expense(Expense),
    Loan(Loan),
}

impl Transaction {
    pub fn target(&self) -> Target {
        match self {
            Transaction::Income(inc) => inc.target(),
            Transaction::Expense(exp) => exp.target(),
            Transaction::Loan(_) => Target::FixExpense,
        }
    }

    pub fn process(&mut self, month: u32) -> f64 {
        match self {
            Transaction::Income(inc) => inc.process(month),
            Transaction::Expense(exp) => exp.process(month),
            Transaction::Loan(loan) => loan.process(),
        }
    }
}
