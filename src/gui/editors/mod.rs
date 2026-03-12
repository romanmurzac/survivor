//! The `editors` module provides modular interfaces for modifying simulation data.
//!
//! Each sub-module within this directory handles the rendering and state
//! synchronization for specific domain objects, allowing the application to
//! maintain a clean, componentized GUI architecture.

/// UI components for managing recurring and unpredictable expenses.
pub mod expenses_editor;

/// UI components for managing active and passive income streams.
pub mod incomes_editor;

/// UI components for tracking and adjusting debt amortization schedules.
pub mod loans_editor;

/// UI components for defining time-based financial interventions and rules.
pub mod scenarios_editor;
