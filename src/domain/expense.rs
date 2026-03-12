//! This module defines the core `Expense` data structure and its
//! associated logic for financial tracking and simulation.

use crate::domain::scenario::Target;

/// Represents the classification and recurrence pattern of an expense.
#[derive(Clone, Copy, Debug)]
pub enum ExpenseType {
    /// Expense occurs every single month.
    Fix,
    /// Expense occurs on a semi-annual basis, every 6 months.
    Variable,
    /// Expense is triggered only in a specific target month.
    Unpredictable(u32),
}

/// Represents an individual financial outgoing, including its classification and amount.
#[derive(Clone, Debug)]
pub struct Expense {
    /// The descriptive name of the expense.
    pub name: String,
    /// The monetary value associated with the expense.
    pub amount: f64,
    /// The frequency pattern of the expense.
    pub frequency: ExpenseType,
}

impl Expense {
    /// Creates a new `Expense`.
    ///
    /// # Arguments
    ///
    /// * `name` - The label for the expense.
    /// * `amount` - The cost value.
    /// * `frequency` - How often this expense is incurred.
    pub fn new(name: impl Into<String>, amount: f64, frequency: ExpenseType) -> Self {
        Self {
            name: name.into(),
            amount,
            frequency,
        }
    }

    /// Calculates the amount incurred for a specific month.
    ///
    /// # Examples
    ///
    /// ```
    /// use domain::expense::{Expense, ExpenseType};
    ///
    /// let fixed = Expense::new("Rent", 1000.0, ExpenseType::Fix);
    /// assert_eq!(fixed.process(1), 1000.0);
    ///
    /// let unpredictable = Expense::new("Car Repair", 500.0, ExpenseType::Unpredictable(3));
    /// assert_eq!(unpredictable.process(3), 500.0);
    /// assert_eq!(unpredictable.process(4), 0.0);
    /// ```
    pub fn process(&self, month: u32) -> f64 {
        match self.frequency {
            ExpenseType::Fix => self.amount,
            ExpenseType::Variable => {
                if month % 6 == 0 {
                    self.amount
                } else {
                    0.0
                }
            }
            ExpenseType::Unpredictable(target_month) => {
                if month == target_month {
                    self.amount
                } else {
                    0.0
                }
            }
        }
    }

    /// Maps the `ExpenseType` to the corresponding system `Target` for financial reporting.
    pub fn target(&self) -> Target {
        match self.frequency {
            ExpenseType::Fix => Target::FixExpense,
            ExpenseType::Variable => Target::VariableExpense,
            ExpenseType::Unpredictable(_) => Target::UnpredictableExpense,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_expense_always_returns_amount() {
        let expense = Expense::new("Rent", 1200.0, ExpenseType::Fix);
        assert_eq!(expense.process(1), 1200.0);
        assert_eq!(expense.process(12), 1200.0);
    }

    #[test]
    fn test_variable_expense_only_every_six_months() {
        let expense = Expense::new("Insurance", 300.0, ExpenseType::Variable);
        assert_eq!(expense.process(6), 300.0);
        assert_eq!(expense.process(12), 300.0);
        assert_eq!(expense.process(1), 0.0);
    }

    #[test]
    fn test_unpredictable_expense_triggers_correct_month() {
        let expense = Expense::new("Car Tax", 250.0, ExpenseType::Unpredictable(5));
        assert_eq!(expense.process(5), 250.0);
        assert_eq!(expense.process(4), 0.0);
    }
}
