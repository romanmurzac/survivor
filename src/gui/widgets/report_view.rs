use egui::Ui;

use crate::report::report::Report;

pub fn render(ui: &mut Ui, reports: &[Report]) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        egui::Grid::new("report_table")
            .striped(true)
            .show(ui, |ui| {
                ui.label("Month");
                ui.label("Savings");
                ui.label("Incomes");
                ui.label("Expenses");
                ui.label("Loans");
                ui.end_row();

                for report in reports {
                    ui.label(report.month.to_string());
                    ui.label(format!("{:.0}", report.savings));
                    ui.label(format!("{:.0}", report.incomes));
                    ui.label(format!("{:.0}", report.expenses));
                    ui.label(format!("{:.0}", report.loans));
                    ui.end_row();
                }
            });
    });
}
