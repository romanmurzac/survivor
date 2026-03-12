//! This module defines the simulation's intervention logic.
//!
//! Scenarios allow for dynamic changes to financial values over
//! time based on predefined rules and targets.

/// Defines the mathematical operation to perform on a financial value.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Reduces the value to zero.
    Loss,
    /// Reduces the value by a given percentage.
    Cut(f64),
    /// Increases the value by a given percentage.
    Increase(f64),
}

/// Identifies which category of financial data a rule should be applied to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    /// Relates to `IncomeCategory::Active`.
    ActiveIncome,
    /// Relates to `IncomeCategory::Passive`.
    PassiveIncome,
    /// Relates to `ExpenseType::Fix`.
    FixExpense,
    /// Relates to `ExpenseType::Variable`.
    VariableExpense,
    /// Relates to `ExpenseType::Unpredictable`.
    UnpredictableExpense,
}

/// A specific instruction to modify a target's value starting from a certain point in time.
#[derive(Clone, Debug)]
pub struct Rule {
    /// The month index when this rule starts taking effect.
    pub start_month: u32,
    /// The financial category this rule affects.
    pub target: Target,
    /// The transformation to apply.
    pub action: Action,
}

/// A collection of rules that define a specific "What If" simulation.
#[derive(Clone, Debug)]
pub struct Scenario {
    /// The list of rules to be evaluated during the simulation.
    pub rules: Vec<Rule>,
}

impl Action {
    /// Applies the action's logic to a numeric value.
    ///
    /// # Arguments
    /// * `value` - The original monetary amount.
    ///
    /// # Examples
    /// ```
    /// # use crate::domain::scenario::Action;
    /// let action = Action::Increase(10.0);
    /// assert_eq!(action.apply(100.0), 110.0);
    /// ```
    pub fn apply(&self, value: f64) -> f64 {
        match self {
            Action::Loss => 0.0,
            Action::Cut(percentage) => value * (100.0 - percentage) / 100.0,
            Action::Increase(percentage) => value * (100.0 + percentage) / 100.0,
        }
    }
}

impl Scenario {
    /// Iterates through all rules and modifies the provided value if the month and target match.
    ///
    /// Note: Rules are additive. If multiple rules apply to the same target in the same month,
    /// they will be applied sequentially in the order they appear in the `rules` vector.
    ///
    /// # Arguments
    /// * `month` - The current simulation month.
    /// * `target` - The category of the value being processed.
    /// * `value` - The base amount before scenario modifications.
    pub fn modify(&self, month: u32, target: Target, value: f64) -> f64 {
        let mut result = value;

        for rule in &self.rules {
            if month >= rule.start_month && rule.target == target {
                result = rule.action.apply(result);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_math() {
        assert_eq!(Action::Loss.apply(100.0), 0.0);
        assert_eq!(Action::Cut(20.0).apply(100.0), 80.0);
        assert_eq!(Action::Increase(50.0).apply(100.0), 150.0);
    }

    #[test]
    fn test_scenario_timing() {
        let scenario = Scenario {
            rules: vec![Rule {
                start_month: 12,
                target: Target::ActiveIncome,
                action: Action::Increase(10.0),
            }],
        };

        // Before month 12.
        assert_eq!(scenario.modify(11, Target::ActiveIncome, 1000.0), 1000.0);
        // At month 12.
        assert_eq!(scenario.modify(12, Target::ActiveIncome, 1000.0), 1100.0);
    }

    #[test]
    fn test_rule_stacking() {
        let scenario = Scenario {
            rules: vec![
                Rule {
                    start_month: 1,
                    target: Target::ActiveIncome,
                    action: Action::Cut(10.0),
                },
                Rule {
                    start_month: 1,
                    target: Target::ActiveIncome,
                    action: Action::Cut(10.0),
                },
            ],
        };

        // 1000 - 10% = 900. Then 900 - 10% = 810.
        assert_eq!(scenario.modify(1, Target::ActiveIncome, 1000.0), 810.0);
    }
}
