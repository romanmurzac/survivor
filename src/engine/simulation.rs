use crate::{
    domain::{scenario::Scenario, status::Status, transaction::Transaction},
    report::report::Report,
};

pub struct Simulation {
    pub transactions: Vec<Transaction>,
    pub scenario: Scenario,
    pub savings: f64,
    pub horizon: u32,
    pub target: f64,
}

impl Simulation {
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
                let value = tx.process(month);
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

            if savings < 0.0 && bankrupt_month.is_none() {
                bankrupt_month = Some(month);
            }

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

            if last_balance < first_balance {
                Status::StillAlive
            } else {
                Status::InTheGame
            }
        };

        (reports, status)
    }
}
