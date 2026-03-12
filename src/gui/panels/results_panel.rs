//! The `results_panel` module handles the visualization of simulation outcomes.
//!
//! This panel integrates three key components:
//! 1. **Status**: A high-level summary of the simulation's success or failure.
//! 2. **Chart**: A visual representation of the cash flow timeline.
//! 3. **Report**: A detailed breakdown of the simulation data.

use egui::Ui;

use crate::gui::app::CashflowApp;
use crate::gui::widgets::{chart_view, report_view};

/// Renders the results dashboard.
///
/// This function pulls the generated data from the `CashflowApp` state and
/// directs it to the appropriate widget renderers.
///
/// # Arguments
/// * `ui` - The `egui::Ui` context.
/// * `app` - A mutable reference to the `CashflowApp` state.
pub fn render(ui: &mut Ui, app: &mut CashflowApp) {
    // Section 1: Final Simulation Status.
    ui.add_space(10.0);
    ui.heading("Status");
    ui.separator();

    ui.label(
        egui::RichText::new(&app.status)
            .size(17.0)
            .color(egui::Color32::DARK_GREEN),
    );
    ui.separator();

    // Section 2: Visual Timeline.
    ui.add_space(10.0);
    ui.heading("Chart");
    ui.separator();

    chart_view::render(ui, &app.reports);
    ui.separator();

    // Section 3: Data Table.
    ui.add_space(10.0);
    ui.heading("Report");
    ui.separator();

    report_view::render(ui, &app.reports);
}
