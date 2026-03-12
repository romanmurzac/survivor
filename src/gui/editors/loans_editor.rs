//! This module provides the user interface for editing loan details.
//!
//! It allows users to manage a collection of debts, adjusting the remaining
//! balance, principal payment amount, and interest charges for each.

use egui::Ui;

use crate::domain::loan::Loan;

/// Renders the loan editor interface.
///
/// This function provides controls to modify the loan parameters, which
/// directly influence the debt repayment lifecycle in the simulation engine.
///
/// # Arguments
/// * `ui` - The `egui::Ui` context.
/// * `loans` - A mutable reference to the `Vec<Loan>` to be modified.
pub fn render(ui: &mut Ui, loans: &mut Vec<Loan>) {
    ui.heading("Loans");

    let mut remove = None;

    for (index, loan) in loans.iter_mut().enumerate() {
        ui.group(|ui| {
            // Header with delete action.
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut loan.name);

                if ui.button("x").clicked() {
                    remove = Some(index);
                }
            });

            // Loan financial parameters.
            ui.horizontal(|ui| {
                ui.label("Remaining");
                ui.add(egui::DragValue::new(&mut loan.remaining));
                ui.label("Principal");
                ui.add(egui::DragValue::new(&mut loan.principal));
                ui.label("Interest");
                ui.add(egui::DragValue::new(&mut loan.interest));
            });
        });
    }

    if let Some(index) = remove {
        loans.remove(index);
    }

    if ui.button("+ Add Loan").clicked() {
        loans.push(Loan::new("Loan", 10000.0, 300.0, 5.0));
    }
}
