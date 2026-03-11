use crate::domain::scenario::Target;

#[derive(Clone, Copy, Debug)]
pub enum ExpenseType {
    Fix,
    Variable,
    Unpredictable(u32),
}

#[derive(Clone, Debug)]
pub struct Expense {
    pub name: String,
    pub amount: f64,
    pub frequency: ExpenseType,
}

impl Expense {
    pub fn new(name: impl Into<String>, amount: f64, frequency: ExpenseType) -> Self {
        Self {
            name: name.into(),
            amount,
            frequency,
        }
    }

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

    pub fn target(&self) -> Target {
        match self.frequency {
            ExpenseType::Fix => Target::FixExpense,
            ExpenseType::Variable => Target::VariableExpense,
            ExpenseType::Unpredictable(_) => Target::UnpredictableExpense,
        }
    }
}
