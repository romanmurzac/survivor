use egui::Ui;

use crate::domain::scenario::{Action, Rule, Scenario, Target};

pub fn render(ui: &mut Ui, scenario: &mut Scenario) {
    ui.heading("Scenarios");

    let mut remove: Option<usize> = None;

    for (index, rule) in scenario.rules.iter_mut().enumerate() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Start Month");
                ui.add(egui::DragValue::new(&mut rule.start_month).range(1..=600));
                if ui.button("x").clicked() {
                    remove = Some(index);
                }
            });

            ui.horizontal(|ui| {
                ui.label("Target");

                egui::ComboBox::from_id_salt(format!("target{}", index))
                    .selected_text(match rule.target {
                        Target::ActiveIncome => "ActiveIncome",
                        Target::PassiveIncome => "PassiveIncome",
                        Target::FixExpense => "FixExpense",
                        Target::VariableExpense => "VariableExpense",
                        Target::UnpredictableExpense => "UnpredictableExpense",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut rule.target, Target::ActiveIncome, "ActiveIncome");
                        ui.selectable_value(
                            &mut rule.target,
                            Target::PassiveIncome,
                            "PassiveIncome",
                        );
                        ui.selectable_value(&mut rule.target, Target::FixExpense, "FixExpense");
                        ui.selectable_value(
                            &mut rule.target,
                            Target::VariableExpense,
                            "VariableExpense",
                        );
                        ui.selectable_value(
                            &mut rule.target,
                            Target::UnpredictableExpense,
                            "UnpredictableExpense",
                        );
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Action");

                let mut new_action = None;

                match &mut rule.action {
                    Action::Loss => {
                        egui::ComboBox::from_id_salt(format!("action{}", index))
                            .selected_text("Loss")
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(true, "Loss").clicked() {
                                    new_action = Some(Action::Loss);
                                }
                                if ui.selectable_label(false, "Cut").clicked() {
                                    new_action = Some(Action::Cut(10.0));
                                }
                                if ui.selectable_label(false, "Increase").clicked() {
                                    new_action = Some(Action::Increase(10.0));
                                }
                            });
                    }
                    Action::Cut(percent) => {
                        egui::ComboBox::from_id_salt(format!("action{}", index))
                            .selected_text("Cut")
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(false, "Loss").clicked() {
                                    new_action = Some(Action::Loss);
                                }
                                if ui.selectable_label(true, "Cut").clicked() {}
                                if ui.selectable_label(false, "Increase").clicked() {
                                    new_action = Some(Action::Increase(10.0));
                                }
                            });

                        ui.label("%");
                        ui.add(egui::DragValue::new(percent).speed(1.0).range(0.0..=100.0));
                    }
                    Action::Increase(percent) => {
                        egui::ComboBox::from_id_salt(format!("action{}", index))
                            .selected_text("Increase")
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(false, "Loss").clicked() {
                                    new_action = Some(Action::Loss);
                                }
                                if ui.selectable_label(true, "Cut").clicked() {
                                    new_action = Some(Action::Cut(10.0));
                                }
                                if ui.selectable_label(false, "Increase").clicked() {}
                            });

                        ui.label("%");
                        ui.add(egui::DragValue::new(percent).speed(1.0).range(0.0..=500.0));
                    }
                }

                if let Some(act) = new_action {
                    rule.action = act;
                }
            });
        });
        ui.separator();
    }
    if let Some(index) = remove {
        scenario.rules.remove(index);
    }
    if ui.button("+ Add Rule").clicked() {
        scenario.rules.push(Rule {
            start_month: 1,
            target: Target::ActiveIncome,
            action: Action::Loss,
        });
    }
}
