use egui::Ui;

use crate::domain::expense::{Expense, ExpenseType};

pub fn render(ui: &mut Ui, expenses: &mut Vec<Expense>) {
    ui.heading("Expenses");

    let mut remove = None;

    for (index, expense) in expenses.iter_mut().enumerate() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut expense.name);

                if ui.button("x").clicked() {
                    remove = Some(index);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Amount");
                ui.add(egui::DragValue::new(&mut expense.amount));
            });

            ui.horizontal(|ui| {
                ui.label("Type");

                let mut new_frequency = None;

                match &mut expense.frequency {
                    ExpenseType::Fix => {
                        egui::ComboBox::from_id_salt(format!("exp_type{}", index))
                            .selected_text("Fix")
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(true, "Fix").clicked() {
                                    new_frequency = Some(ExpenseType::Fix);
                                }
                                if ui.selectable_label(false, "Variable").clicked() {
                                    new_frequency = Some(ExpenseType::Variable);
                                }
                                if ui.selectable_label(false, "Unpredictable").clicked() {
                                    new_frequency = Some(ExpenseType::Unpredictable(1));
                                }
                            });
                    }

                    ExpenseType::Variable => {
                        egui::ComboBox::from_id_salt(format!("exp_type{}", index))
                            .selected_text("Variable")
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(false, "Fix").clicked() {
                                    new_frequency = Some(ExpenseType::Fix);
                                }
                                if ui.selectable_label(true, "Variable").clicked() {
                                    new_frequency = Some(ExpenseType::Variable);
                                }
                                if ui.selectable_label(false, "Unpredictable").clicked() {
                                    new_frequency = Some(ExpenseType::Unpredictable(1));
                                }
                            });
                    }

                    ExpenseType::Unpredictable(month) => {
                        egui::ComboBox::from_id_salt(format!("exp_type{}", index))
                            .selected_text("Fix")
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(false, "Fix").clicked() {
                                    new_frequency = Some(ExpenseType::Fix);
                                }
                                if ui.selectable_label(false, "Variable").clicked() {
                                    new_frequency = Some(ExpenseType::Variable);
                                }
                                if ui.selectable_label(true, "Unpredictable").clicked() {}
                            });

                        ui.label("Month");
                        ui.add(egui::DragValue::new(month).range(1..=600));
                    }
                }

                if let Some(freq) = new_frequency {
                    expense.frequency = freq;
                }
            });
        });
    }

    if let Some(index) = remove {
        expenses.remove(index);
    }

    if ui.button("+ Add Expense").clicked() {
        expenses.push(Expense::new("Expense", 500.0, ExpenseType::Fix));
    }
}
