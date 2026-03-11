use crate::domain::scenario::Target;

#[derive(Clone, Copy, Debug)]
pub enum IncomeType {
    Recurrent,
    OneTime(u32),
}

#[derive(Clone, Copy, Debug)]
pub enum IncomeCategory {
    Active,
    Passive,
}

#[derive(Clone, Debug)]
pub struct Income {
    pub name: String,
    pub amount: f64,
    pub frequency: IncomeType,
    pub category: IncomeCategory,
}

impl Income {
    pub fn new(name: impl Into<String>, amount: f64, frequency: IncomeType, category: IncomeCategory) -> Self {
        Self {
            name: name.into(),
            amount,
            frequency,
            category,
        }
    }

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

    pub fn target(&self) -> Target {
        match self.category {
            IncomeCategory::Active => Target::ActiveIncome,
            IncomeCategory::Passive => Target::PassiveIncome,
        }
    }
}
