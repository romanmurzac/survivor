//! This module provides terminal-based data previewing.
//!
//! It is primarily used to quickly inspect the
//! structure and content of `Report` snapshots directly from the command line,
//! bypassing the need for a full GUI build.

use crate::report::report::Report;

/// Prints a formatted table of simulation reports to the console.
///
/// This function uses fixed-width formatting to ensure that data columns
/// remain aligned, making it easy to spot anomalies in financial
/// calculations at a glance.
///
/// # Arguments
/// * `reports` - A slice of `Report` snapshots to display.
#[allow(dead_code)]
pub fn preview_report(reports: &[Report]) {
    // Header for the terminal output.
    println!(
        "{:<6} {:<10} {:<12} {:<14} {:<10}",
        "Month", "Savings", "Incomes", "Expenses", "Loans"
    );

    // Row iteration.
    for report in reports {
        println!(
            "{:<6} {:<10.0} {:<12.0} {:<14.0} {:<10.0}",
            report.month, report.savings, report.incomes, report.expenses, report.loans
        );
    }
}
