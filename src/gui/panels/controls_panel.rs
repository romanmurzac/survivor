//! The `controls_panel` module provides the primary dashboard for user input.
//!
//! This panel integrates global simulation parameters with modular data editors.
//! It serves as the main entry point for users to configure their financial
//! scenario, income sources, expenses, and debt obligations before running
//! the simulation engine.

use chrono::Local;
use egui::Ui;

use crate::gui::app::CashflowApp;
use crate::gui::editors::{expenses_editor, incomes_editor, loans_editor, scenarios_editor};
use crate::utils::export::{download_csv, export_csv};

/// Renders the main simulation configuration panel.
///
/// This function constructs the GUI layout by calling the specialized renderers
/// defined in the `editors` module.
///
/// # Arguments
/// * `ui` - The `egui::Ui` context.
/// * `app` - A mutable reference to the `CashflowApp` state, which holds
///   all simulation data and configuration.
pub fn render(ui: &mut Ui, app: &mut CashflowApp) {
    ui.add_space(10.0);
    ui.heading("Simulation");

    // Global Simulation Parameters.
    ui.add_space(3.0);
    ui.add(egui::Slider::new(&mut app.savings, 0.0..=1_000_000.0).text("Savings"));

    ui.add_space(3.0);
    ui.add(egui::Slider::new(&mut app.horizon, 1..=600).text("Horizon"));

    ui.add_space(3.0);
    ui.add(egui::Slider::new(&mut app.target, 0.0..=5_000_000.0).text("Target"));
    ui.separator();

    // Modular Editor Components.
    ui.add_space(10.0);
    scenarios_editor::render(ui, &mut app.scenario);
    ui.separator();

    ui.add_space(10.0);
    incomes_editor::render(ui, &mut app.incomes);
    ui.separator();

    ui.add_space(10.0);
    expenses_editor::render(ui, &mut app.expenses);
    ui.separator();

    ui.add_space(10.0);
    loans_editor::render(ui, &mut app.loans);
    ui.separator();

    // Export Processing.
    ui.add_space(10.0);
    ui.heading("Data Process");
    if ui.button("Download CSV Report").clicked() {
        let csv_file = export_csv(&app.reports);
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let filename = format!("survivor_report_{}.csv", timestamp);
        download_csv(&filename, &csv_file);
    }
}
