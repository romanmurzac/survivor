use egui::Ui;

use crate::domain::loan::Loan;

pub fn render(ui: &mut Ui, loans: &mut Vec<Loan>) {
    ui.heading("Loans");

    let mut remove = None;

    for (index, loan) in loans.iter_mut().enumerate() {
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.text_edit_singleline(&mut loan.name);

            if ui.button("x").clicked() {
                remove = Some(index);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Remaining");
            ui.add(egui::DragValue::new(&mut loan.remaining));
            ui.label("Principal");
            ui.add(egui::DragValue::new(&mut loan.principal));
            ui.label("Interest");
            ui.add(egui::DragValue::new(&mut loan.interest));
        });
    }

    if let Some(index) = remove {
        loans.remove(index);
    }

    if ui.button("+ Add Loan").clicked() {
        loans.push(Loan::new("Loan", 10000.0, 300.0, 5.0));
    }
}
