use eframe::egui;

use super::panels::{controls_panel, results_panel};
use crate::domain::{expense::Expense, income::Income, loan::Loan, scenario::Scenario};
use crate::report::report::Report;

pub struct CashflowApp {
    pub savings: f64,
    pub horizon: u32,
    pub target: f64,

    pub incomes: Vec<Income>,
    pub expenses: Vec<Expense>,
    pub loans: Vec<Loan>,

    pub scenario: Scenario,

    pub reports: Vec<Report>,
    pub status: String,
}

impl Default for CashflowApp {
    fn default() -> Self {
        Self {
            savings: 5000.0,
            horizon: 24,
            target: 50000.0,
            incomes: vec![],
            expenses: vec![],
            loans: vec![],
            scenario: Scenario { rules: vec![] },
            reports: vec![],
            status: String::new(),
        }
    }
}

impl eframe::App for CashflowApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("controls").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                controls_panel::render(ui, self);
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            results_panel::render(ui, self);
        });
        self.recompute();
    }
}

impl CashflowApp {
    fn recompute(&mut self) {
        use crate::domain::transaction::Transaction;
        use crate::engine::simulation::Simulation;

        let mut transactions = vec![];

        for income in &self.incomes {
            transactions.push(Transaction::Income(income.clone()));
        }
        for expense in &self.expenses {
            transactions.push(Transaction::Expense(expense.clone()));
        }
        for loan in &self.loans {
            transactions.push(Transaction::Loan(loan.clone()));
        }

        let mut sim = Simulation {
            savings: self.savings,
            horizon: self.horizon,
            target: self.target,
            transactions,
            scenario: self.scenario.clone(),
        };

        let (reports, status) = sim.run();

        self.reports = reports;
        self.status = status.format_status();
    }
}
