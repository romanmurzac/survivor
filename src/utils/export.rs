use crate::report::report::Report;

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

    let array = js_sys::Array::new();
    array.push(&wasm_bindgen::JsValue::from_str(content));

    let blob = Blob::new_with_str_sequence(&array).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();

    let document = web_sys::window().unwrap().document().unwrap();

    let a = document
        .create_element("a")
        .unwrap()
        .dyn_into::<HtmlAnchorElement>()
        .unwrap();

    a.set_href(&url);
    a.set_download(filename);
    a.click();

    Url::revoke_object_url(&url).unwrap();
}
