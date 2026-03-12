use egui::Ui;
use egui_plot::{Bar, BarChart, Plot};

use crate::report::report::Report;

pub fn render(ui: &mut Ui, reports: &[Report]) {
    let bars: Vec<Bar> = reports
        .iter()
        .map(|report| Bar::new(report.month as f64, report.savings))
        .collect();

    let chart = BarChart::new("Char", bars).width(0.6);

    Plot::new("savings_chart")
        .height(300.0)
        .show_grid(false)
        .show_axes([true, true])
        .x_axis_label("Month")
        .y_axis_label("Balance")
        .allow_scroll(false)
        .show_background(false)
        .y_axis_formatter(|grid_mark, _max_chars| format!("{:.0}", grid_mark.value))
        .show(ui, |plot_ui| {
            plot_ui.bar_chart(chart);
        });
}
