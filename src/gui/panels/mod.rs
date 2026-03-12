//! The `panels` module defines the high-level layout components of the GUI.
//!
//! This module acts as the container for the application's major views,
//! coordinating the interaction between user inputs (via `controls_panel`)
//! and data visualization (via `results_panel`).

/// The main dashboard for configuring simulation variables and managing data.
pub mod controls_panel;

/// The display area for viewing simulation outcomes, charts, and reports.
pub mod results_panel;
