//! The `gui` module manages the application's user interface and state lifecycle.
//!
//! This module acts as the frontend layer, connecting user inputs to the
//! underlying `domain` and `engine` logic. It is structured into:
//!
//! * **app**: The main entry point and global state manager.
//! * **editors**: Reusable input forms for modifying domain models.
//! * **panels**: The high-level layout containers that structure the UI.
//! * **widgets**: Primitive rendering components for charts and reports.

/// The central state container and `eframe` application logic.
pub mod app;

/// Components for editing financial data (Incomes, Expenses, Loans, Scenarios).
pub mod editors;

/// Structural layout panels (Controls and Results dashboards).
pub mod panels;

/// Reusable rendering widgets for charts and data tables.
pub mod widgets;
