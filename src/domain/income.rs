//! This module defines the `Income` structures and logic.
//!
//! It categorizes different revenue streams and handles
//! their temporal distribution across a simulation timeline.

use crate::domain::scenario::Target;

/// Defines the temporal pattern of an income stream.
#[derive(Clone, Copy, Debug)]
pub enum IncomeType {
    /// Income that is received every month.
    Recurrent,
    /// Income that occurs exactly once in a specific month.
    OneTime(u32),
}

/// Categorizes the nature of the income for financial strategy analysis.
#[derive(Clone, Copy, Debug)]
pub enum IncomeCategory {
    /// Income derived from direct labor or business activity.
    Active,
    /// Income derived from investments or assets.
    Passive,
}

/// Represents a financial inflow.
#[derive(Clone, Debug)]
pub struct Income {
    /// The name or source of the income.
    pub name: String,
    /// The monetary value of the inflow.
    pub amount: f64,
    /// How often this income is realized.
    pub frequency: IncomeType,
    /// The classification of the income source.
    pub category: IncomeCategory,
}

impl Income {
    /// Creates a new `Income` instance.
    ///
    /// # Arguments
    ///
    /// * `name` - The label for the income source.
    /// * `amount` - The monetary amount.
    /// * `frequency` - Whether it repeats or is a one-off.
    /// * `category` - Whether it is active or passive.
    pub fn new(
        name: impl Into<String>,
        amount: f64,
        frequency: IncomeType,
        category: IncomeCategory,
    ) -> Self {
        Self {
            name: name.into(),
            amount,
            frequency,
            category,
        }
    }

    /// Calculates the income amount realized for a specific month.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::domain::income::{Income, IncomeType, IncomeCategory};
    /// let salary = Income::new("Salary", 3000.0, IncomeType::Recurrent, IncomeCategory::Active);
    /// assert_eq!(salary.process(1), 3000.0);
    /// assert_eq!(salary.process(100), 3000.0);
    /// ```
    pub fn process(&self, month: u32) -> f64 {
        match self.frequency {
            IncomeType::Recurrent => self.amount,
            IncomeType::OneTime(target_month) => {
                if month == target_month {
                    self.amount
                } else {
                    0.0
                }
            }
        }
    }

    /// Maps the income category to a simulation `Target`.
    pub fn target(&self) -> Target {
        match self.category {
            IncomeCategory::Active => Target::ActiveIncome,
            IncomeCategory::Passive => Target::PassiveIncome,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurrent_income_always_hits() {
        let income = Income::new(
            "Salary",
            5000.0,
            IncomeType::Recurrent,
            IncomeCategory::Active,
        );
        assert_eq!(income.process(1), 5000.0);
        assert_eq!(income.process(12), 5000.0);
        assert_eq!(income.process(99), 5000.0);
    }

    #[test]
    fn test_one_time_income_hits_only_once() {
        let bonus_month = 12;
        let income = Income::new(
            "Bonus",
            1000.0,
            IncomeType::OneTime(bonus_month),
            IncomeCategory::Active,
        );

        assert_eq!(income.process(bonus_month), 1000.0);
        assert_eq!(income.process(bonus_month - 1), 0.0);
        assert_eq!(income.process(bonus_month + 1), 0.0);
    }

    #[test]
    fn test_income_targeting() {
        let active = Income::new("Job", 100.0, IncomeType::Recurrent, IncomeCategory::Active);
        let passive = Income::new(
            "Stocks",
            100.0,
            IncomeType::Recurrent,
            IncomeCategory::Passive,
        );

        assert_eq!(active.target(), Target::ActiveIncome);
        assert_eq!(passive.target(), Target::PassiveIncome);
    }
}
