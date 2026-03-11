use chrono::Local;
use egui::Ui;

use crate::gui::app::CashflowApp;
use crate::gui::editors::{expenses_editor, incomes_editor, loans_editor, scenarios_editor};
use crate::utils::export::{download_csv, export_csv};

pub fn render(ui: &mut Ui, app: &mut CashflowApp) {
    ui.add_space(10.0);

    ui.heading("Simulation");

    ui.add_space(3.0);
    ui.add(egui::Slider::new(&mut app.savings, 0.0..=1_000_000.0).text("Savings"));

    ui.add_space(3.0);
    ui.add(egui::Slider::new(&mut app.horizon, 1..=600).text("Horizon"));

    ui.add_space(3.0);
    ui.add(egui::Slider::new(&mut app.target, 0.0..=2_000_000.0).text("Target"));
    ui.separator();

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

    ui.add_space(10.0);
    ui.heading("Process");
    if ui.button("Download CSV Report").clicked() {
        let csv_file = export_csv(&app.reports);
        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let filename = format!("survivor_report_{}.csv", timestamp);
        download_csv(&filename, &csv_file);
    }
}
