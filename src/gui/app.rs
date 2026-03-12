//! The primary application entry point.
//!
//! `CashflowApp` maintains the complete application state, including financial
//! configuration, simulation input, and the computed output.

use eframe::egui;

use super::panels::{controls_panel, results_panel};
use crate::domain::{expense::Expense, income::Income, loan::Loan, scenario::Scenario};
use crate::report::report::Report;

/// The central state container for the financial simulator.
pub struct CashflowApp {
    // --- Global Configuration ---
    pub savings: f64,
    pub horizon: u32,
    pub target: f64,

    // --- Domain Data ---
    pub incomes: Vec<Income>,
    pub expenses: Vec<Expense>,
    pub loans: Vec<Loan>,
    pub scenario: Scenario,

    // --- Computed Output ---
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
    /// The core update loop, executed every frame.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Layout: Left panel for settings.
        egui::SidePanel::left("controls").show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                controls_panel::render(ui, self);
            });
        });

        // Layout: Central panel for visualization.
        egui::CentralPanel::default().show(ctx, |ui| {
            results_panel::render(ui, self);
        });
        self.recompute();
    }
}

impl CashflowApp {
    fn recompute(&mut self) {
        /// Bridges the GUI state with the Simulation engine.
        ///
        /// This method clones the current domain objects into the simulation,
        /// executes the engine's `run` method, and updates the app's state
        /// with the resulting reports.
        use crate::domain::transaction::Transaction;
        use crate::engine::simulation::Simulation;

        // Flatten individual collections into a unified list of Transactions.
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

        // Configure the simulation engine.
        let mut sim = Simulation {
            savings: self.savings,
            horizon: self.horizon,
            target: self.target,
            transactions,
            scenario: self.scenario.clone(),
        };

        // Execute engine.
        let (reports, status) = sim.run();

        // Update app state for UI rendering.
        self.reports = reports;
        self.status = status.format_status();
    }
}
