//! The `engine` module contains the computational logic of the application.
//! It defines *how* they evolve over time. It is responsible for:
//!
//! 1.  **State Management**: Tracking the fluctuating balance of savings and loans.
//! 2.  **Temporal Iteration**: Walking through the simulation month-by-month.
//! 3.  **Reporting**: Producing snapshots of the financial state at every step.

/// The primary simulation runner and its associated configuration.
pub mod simulation;
