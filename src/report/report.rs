//! This module defines the `Report` data structure.
//!
//! A `Report` acts as a historical snapshot of the simulation state.
//! By collecting these snapshots over the simulation horizon, the
//! application can visualize financial trends and generate detailed logs.

#[derive(Debug)]
pub struct Report {
    /// The month index of this snapshot.
    pub month: u32,
    /// Total accumulated savings at this month.
    pub savings: f64,
    /// Total income realized in this month.
    pub incomes: f64,
    /// Total expenses incurred in this month.
    pub expenses: f64,
    /// Total loan repayments processed in this month.
    pub loans: f64,
}

impl Report {
    /// Creates a new temporal snapshot of the financial state.
    ///
    /// # Arguments
    /// * `month` - The current simulation month.
    /// * `savings` - Cumulative balance.
    /// * `incomes` - Inflow for the current month.
    /// * `expenses` - Outflow for the current month.
    /// * `loans` - Debt repayment for the current month.
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
