use crate::report::report::Report;

#[allow(dead_code)]
pub fn preview_report(reports: &[Report]) {
    println!(
        "{:<6} {:<10} {:<12} {:<14} {:<10}",
        "Month", "Savings", "Incomes", "Expenses", "Loans"
    );
    for report in reports {
        println!(
            "{:<6} {:<10.0} {:<12.0} {:<14.0} {:<10.0}",
            report.month, report.savings, report.incomes, report.expenses, report.loans
        );
    }
}
