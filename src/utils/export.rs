//! This module provides cross-platform CSV export capabilities.
//!
//! It enables the conversion of `Report` snapshots into CSV strings
//! and provides platform-specific mechanisms to save that data to a file.

use crate::report::report::Report;

/// Converts a slice of `Report` snapshots into a formatted CSV string.
///
/// # Arguments
/// * `reports` - A slice of financial snapshots to serialize.
pub fn export_csv(reports: &[Report]) -> String {
    let mut csv_file = String::from("Month,Savings,Incomes,Expenses,Loans\n");

    for report in reports {
        csv_file.push_str(&format!(
            "{},{:.0},{:.0},{:.0},{:.0}\n",
            report.month, report.savings, report.incomes, report.expenses, report.loans
        ));
    }
    csv_file
}

/// Downloads or saves a CSV string to the local system.
///
/// This function uses `#[cfg]` conditional compilation to provide the correct
/// implementation based on the target architecture:
/// * **Native**: Writes the file directly to the disk.
/// * **WASM**: Triggers a browser download link.
#[cfg(not(target_arch = "wasm32"))]
pub fn download_csv(filename: &str, content: &str) {
    use std::fs::File;
    use std::io::Write;

    let mut file = File::create(filename).expect("Failed to create file.");
    file.write_all(content.as_bytes())
        .expect("Failed to write file.")
}

#[cfg(target_arch = "wasm32")]
pub fn download_csv(filename: &str, content: &str) {
    use wasm_bindgen::JsCast;
    use web_sys::{Blob, HtmlAnchorElement, Url};

    // Prepare data as a Blob.
    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::JsValue::from_str(content));

    let blob = Blob::new_with_str_sequence(&array).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    // Browser-side simulation of a file download link.
    let document = web_sys::window().unwrap().document().unwrap();

    let a = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();

    a.set_href(&url);
    a.set_download(filename);
    a.click();

    // Cleanup object URL.
    Url::revoke_object_url(&url).unwrap();
}
