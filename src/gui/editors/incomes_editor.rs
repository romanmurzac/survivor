//! This module renders the user interface for managing income sources.
//!
//! It allows users to manipulate `Income` data in real-time, including
//! switching between `Recurrent` and `OneTime` frequencies and classifying
//! streams as `Active` or `Passive`.

use egui::Ui;

use crate::domain::income::{Income, IncomeCategory, IncomeType};

/// Renders the interactive editor for a list of incomes.
///
/// # Arguments
/// * `ui` - The `egui::Ui` context.
/// * `incomes` - A mutable reference to the `Vec<Income>` to be modified.
pub fn render(ui: &mut Ui, incomes: &mut Vec<Income>) {
    ui.heading("Incomes");

    let mut remove = None;

    for (index, income) in incomes.iter_mut().enumerate() {
        ui.group(|ui| {
            // Header: Name and deletion controls.
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut income.name);

                if ui.button("x").clicked() {
                    remove = Some(index);
                }
            });

            // Numeric value input.
            ui.horizontal(|ui| {
                ui.label("Amount");
                ui.add(egui::DragValue::new(&mut income.amount));
            });

            // Frequency Configuration.
            ui.horizontal(|ui| {
                ui.label("Type");

                let mut new_frequency = None;

                match &mut income.frequency {
                    IncomeType::Recurrent => {
                        egui::ComboBox::from_id_salt(format!("income_type{}", index))
                            .selected_text("Recurrent")
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(true, "Recurrent").clicked() {
                                    new_frequency = Some(IncomeType::Recurrent);
                                }
                                if ui.selectable_label(false, "OneTime").clicked() {
                                    new_frequency = Some(IncomeType::OneTime(1));
                                }
                            });
                    }

                    IncomeType::OneTime(month) => {
                        egui::ComboBox::from_id_salt(format!("income_type{}", index))
                            .selected_text("OneTime")
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(false, "Recurrent").clicked() {
                                    new_frequency = Some(IncomeType::Recurrent);
                                }
                                if ui.selectable_label(true, "OneTime").clicked() {};
                            });

                        ui.label("Month");
                        ui.add(egui::DragValue::new(month).range(1..=600));
                    }
                }

                if let Some(freq) = new_frequency {
                    income.frequency = freq;
                }
            });

            // Category Selection.
            ui.horizontal(|ui| {
                ui.label("Category");

                egui::ComboBox::from_id_salt(format!("income_cat{}", index))
                    .selected_text(match income.category {
                        IncomeCategory::Active => "Active",
                        IncomeCategory::Passive => "Passive",
                    })
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(false, "Active").clicked() {
                            income.category = IncomeCategory::Active;
                        }
                        if ui.selectable_label(true, "Passive").clicked() {
                            income.category = IncomeCategory::Passive;
                        }
                    });
            });
        });
    }

    if let Some(index) = remove {
        incomes.remove(index);
    }

    if ui.button("+ Add Income").clicked() {
        incomes.push(Income::new(
            "Income",
            1000.0,
            IncomeType::Recurrent,
            IncomeCategory::Active,
        ));
    }
}
