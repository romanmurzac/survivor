//! The core execution engine of the financial simulation.
//!
//! This module coordinates the interaction between financial transactions,
//! "what-if" scenarios, and the accumulation of savings over a defined time horizon.

use crate::{
    domain::{scenario::Scenario, status::Status, transaction::Transaction},
    report::report::Report,
};

/// Orchestrates a financial simulation run.
///
/// A simulation tracks how a set of transactions and a specific scenario
/// affect a user's initial savings over a period of months (the horizon).
pub struct Simulation {
    /// The list of income, expense, and loan items to process.
    pub transactions: Vec<Transaction>,
    /// The set of rules that dynamically modify transaction values.
    pub scenario: Scenario,
    /// Starting cash balance.
    pub savings: f64,
    /// Number of months to run the simulation.
    pub horizon: u32,
    /// The savings goal that triggers the `Survivor` status.
    pub target: f64,
}

impl Simulation {
    /// Executes the simulation and returns a timeline of reports and the final status.
    ///
    /// The simulation proceeds month-by-month:
    /// 1. Processes each transaction (applying state changes to loans).
    /// 2. Modifies values based on the active `Scenario`.
    /// 3. Updates the cumulative savings.
    /// 4. Records snapshots in a `Report`.
    /// 5. Evaluates bankruptcy or milestone achievements.
    ///
    /// # Returns
    /// A tuple containing:
    /// * `Vec<Report>`: A month-by-month breakdown of the financial state.
    /// * `Status`: The final outcome (Bankrupt, Survivor, InTheGame, StillAlive).
    pub fn run(&mut self) -> (Vec<Report>, Status) {
        let mut reports = Vec::new();
        let mut savings = self.savings;

        let mut bankrupt_month: Option<u32> = None;
        let mut survivor_month: Option<u32> = None;

        for month in 1..=self.horizon {
            let mut total_incomes = 0.0;
            let mut total_expenses = 0.0;
            let mut total_loans = 0.0;

            for tx in &mut self.transactions {
                // Get the raw value (updates loan state internally).
                let value = tx.process(month);
                // Apply scenario interventions (inflation or pay raises).
                let modified = self.scenario.modify(month, tx.target(), value);

                match tx {
                    Transaction::Income(_) => total_incomes += modified,
                    Transaction::Expense(_) => total_expenses += modified,
                    Transaction::Loan(_) => total_loans += modified,
                }
            }

            let net = total_incomes - total_expenses - total_loans;
            savings += net;

            reports.push(Report::new(
                month,
                savings,
                total_incomes,
                total_expenses,
                total_loans,
            ));

            // Capture first instance of bankruptcy.
            if savings < 0.0 && bankrupt_month.is_none() {
                bankrupt_month = Some(month);
            }

            // Capture first instance of reaching the target goal.
            if savings >= self.target && survivor_month.is_none() {
                survivor_month = Some(month);
            }
        }

        let status = if let Some(month) = bankrupt_month {
            Status::Bankrupt { month }
        } else if let Some(month) = survivor_month {
            Status::Survivor { month }
        } else {
            let first_balance = reports.first().map(|r| r.savings).unwrap_or(self.savings);
            let last_balance = reports.last().map(|r| r.savings).unwrap_or(self.savings);

            // Logic to distinguish between declining and growing wealth.
            if last_balance < first_balance {
                Status::StillAlive
            } else {
                Status::InTheGame
            }
        };

        (reports, status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::expense::{Expense, ExpenseType};

    #[test]
    fn test_basic_bankruptcy() {
        let expense =
            Transaction::Expense(Expense::new("Expensive Habit", 100.0, ExpenseType::Fix));
        let mut sim = Simulation {
            transactions: vec![expense],
            scenario: Scenario { rules: vec![] },
            savings: 150.0, // Should last 2 months.
            horizon: 5,
            target: 1000.0,
        };

        let (reports, status) = sim.run();

        assert_eq!(reports.len(), 5);
        if let Status::Bankrupt { month } = status {
            assert_eq!(month, 2); // 150 - 100 = 50. 50 - 100 = -50 (Month 2).
        } else {
            panic!("Should have been bankrupt!");
        }
    }
}
