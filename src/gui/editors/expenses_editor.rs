//! This module provides the user interface for editing a collection of expenses.
//!
//! It utilizes an immediate-mode pattern to synchronize the `Vec<Expense>` data
//! with the UI state in real-time.

use egui::Ui;

use crate::domain::expense::{Expense, ExpenseType};

/// Renders the expense editor interface.
///
/// This function creates a scrollable or grouped list of expenses, allowing the user to:
/// 1. Edit the name and amount of existing expenses.
/// 2. Change the recurrence type (Fix, Variable, Unpredictable).
/// 3. Remove an existing expense.
/// 4. Add a new default expense to the list.
///
/// # Arguments
/// * `ui` - The `egui::Ui` context for rendering.
/// * `expenses` - A mutable reference to the vector of expenses to be modified.
pub fn render(ui: &mut Ui, expenses: &mut Vec<Expense>) {
    ui.heading("Expenses");

    // Tracks which expense (if any) should be removed after the loop to avoid
    // modifying the vector while iterating over it.
    let mut remove = None;

    for (index, expense) in expenses.iter_mut().enumerate() {
        ui.group(|ui| {
            // Header: Name field and Delete button.
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut expense.name);

                if ui.button("x").clicked() {
                    remove = Some(index);
                }
            });

            // Amount field using DragValue for numeric input.
            ui.horizontal(|ui| {
                ui.label("Amount");
                ui.add(egui::DragValue::new(&mut expense.amount));
            });

            // Frequency Type Selection.
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
                            .selected_text("Unpredictable")
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
                        // Range limited to 600 months (50 years) for practical simulation limits.
                        ui.add(egui::DragValue::new(month).range(1..=600));
                    }
                }

                // Apply frequency changes if the user selected a different variant in the ComboBox.
                if let Some(freq) = new_frequency {
                    expense.frequency = freq;
                }
            });
        });
    }

    // Execute removal outside of the iteration loop.
    if let Some(index) = remove {
        expenses.remove(index);
    }

    // Add a new expense with sensible defaults.
    if ui.button("+ Add Expense").clicked() {
        expenses.push(Expense::new("Expense", 500.0, ExpenseType::Fix));
    }
}
