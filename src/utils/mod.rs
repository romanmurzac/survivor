//! The `utils` module provides helper functions and cross-platform utilities.
//!
//! This module contains non-domain, non-GUI logic that supports the broader
//! application, such as file system/browser export mechanisms and
//! diagnostic CLI tools.

/// Provides functionality for serializing and downloading data as CSV.
pub mod export;

/// Provides console-based debugging utilities for rapid data inspection.
pub mod preview;
