//! The `domain` module represents the core data models of the financial system.
//!
//! This module contains the foundational building blocks used by the simulation
//! engine and the GUI, including:
//! * **Incomes & Expenses**: Definitions for cash flow direction and frequency.
//! * **Loans**: Stateful debt tracking.
//! * **Scenarios**: "What-if" rules that modify financial data over time.
//! * **Transactions**: A unified wrapper for processing all financial items.
//! * **Status**: The resulting state of a simulation run.

/// Logic for outflows and spending patterns.
pub mod expense;

/// Logic for inflows and revenue categorization.
pub mod income;

/// Logic for debt management and amortization.
pub mod loan;

/// Logic for interventions and simulation rules.
pub mod scenario;

/// Logic for evaluating and formatting simulation outcomes.
pub mod status;

/// The polymorphic interface connecting all financial types.
pub mod transaction;
