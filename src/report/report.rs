#[derive(Debug)]
pub struct Report {
    pub month: u32,
    pub savings: f64,
    pub incomes: f64,
    pub expenses: f64,
    pub loans: f64,
}

impl Report {
    pub fn new(month: u32, savings: f64, incomes: f64, expenses: f64, loans: f64) -> Self {
        Self {
            month,
            savings,
            incomes,
            expenses,
            loans,
        }
    }
}
