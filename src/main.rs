//! The main entry point for the "Survivor" financial simulator.
//!
//! This module initializes the application state and selects the appropriate
//! runtime based on the target architecture (Native Desktop or Web/WASM).

mod domain;
mod engine;
mod gui;
mod report;
mod utils;

use gui::app::CashflowApp;

fn main() -> eframe::Result<()> {
    // --- Native Desktop Implementation ---
    #[cfg(not(target_arch = "wasm32"))]
    {
        let options = eframe::NativeOptions {
            viewport: eframe::egui::ViewportBuilder::default()
                .with_inner_size([1200.0, 800.0])
                .with_min_inner_size([900.0, 600.0])
                .with_title("Survivor"),
            ..Default::default()
        };

        eframe::run_native(
            "Survivor",
            options,
            Box::new(|_cc| Ok(Box::new(CashflowApp::default()) as Box<dyn eframe::App>)),
        )
    }

    // --- Web/WASM Implementation ---
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;

        wasm_bindgen_futures::spawn_local(async {
            let document = web_sys::window().unwrap().document().unwrap();

            let canvas = document
                .get_element_by_id("the_canvas_id")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();

            eframe::WebRunner::new()
                .start(
                    canvas,
                    eframe::WebOptions::default(),
                    Box::new(|_cc| Ok(Box::new(CashflowApp::default()))),
                )
                .await
                .expect("failed to start");
        });

        Ok(())
    }
}
