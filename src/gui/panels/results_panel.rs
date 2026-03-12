use egui::Ui;

use crate::gui::app::CashflowApp;
use crate::gui::widgets::{chart_view, report_view};

pub fn render(ui: &mut Ui, app: &mut CashflowApp) {
    ui.add_space(10.0);
    ui.heading("Status");
    ui.separator();

    ui.label(
        egui::RichText::new(&app.status)
            .size(17.0)
            .color(egui::Color32::DARK_GREEN),
    );
    ui.separator();

    ui.add_space(10.0);
    ui.heading("Chart");
    ui.separator();

    chart_view::render(ui, &app.reports);
    ui.separator();

    ui.add_space(10.0);
    ui.heading("Report");
    ui.separator();

    report_view::render(ui, &app.reports);
}
