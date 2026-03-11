#[derive(Clone, Copy, Debug)]
pub enum Action {
    Loss,
    Cut(f64),
    Increase(f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Target {
    ActiveIncome,
    PassiveIncome,
    FixExpense,
    VariableExpense,
    UnpredictableExpense,
}

#[derive(Clone, Debug)]
pub struct Rule {
    pub start_month: u32,
    pub target: Target,
    pub action: Action,
}

#[derive(Clone, Debug)]
pub struct Scenario {
    pub rules: Vec<Rule>,
}

impl Action {
    pub fn apply(&self, value: f64) -> f64 {
        match self {
            Action::Loss => 0.0,
            Action::Cut(percentage) => value * (100.0 - percentage) / 100.0,
            Action::Increase(percentage) => value * (100.0 + percentage) / 100.0,
        }
    }
}

impl Scenario {
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