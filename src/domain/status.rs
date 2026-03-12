//! This module defines the final state or health of a financial simulation.
//!
//! The `Status` enum is used to provide feedback on the success or failure
//! of a specific scenario over its duration.

/// Represents the final outcome of the financial simulation.
#[derive(Debug)]
pub enum Status {
    /// The net balance dropped below zero.
    /// The specific month index where the balance became negative.
    Bankrupt { month: u32 },
    /// The simulation completed without bankruptcy, but with negative balance.
    StillAlive,
    /// The simulation completed without survivor, but with positive balance.
    InTheGame,
    /// Achieved the financial independence setup by target.
    /// The specific month index when the milestone was achieved.
    Survivor { month: u32 },
}

impl Status {
    /// Returns a human-readable description of the status for display in the GUI.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::domain::status::Status;
    /// let status = Status::Bankrupt { month: 12 };
    /// assert_eq!(status.format_status(), "In 12 months you become BANKRUPT.");
    ///
    /// let early_win = Status::Survivor { month: 1 };
    /// assert_eq!(early_win.format_status(), "You are a SURVIVOR from the first month.");
    /// ```
    pub fn format_status(&self) -> String {
        match &self {
            Self::Survivor { month } => {
                if *month == 1 {
                    "You are a SURVIVOR from the first month.".to_string()
                } else {
                    format!("In {} months you become SURVIVOR.", month)
                }
            }
            Self::Bankrupt { month } => {
                if *month == 1 {
                    "You are BANKRUPT in the first month".to_string()
                } else {
                    format!("In {} months you become BANKRUPT.", month)
                }
            }
            Self::InTheGame => "You are InTheGame.".to_string(),
            Self::StillAlive => "You are StillAlive.".to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bankrupt_formatting() {
        assert_eq!(
            Status::Bankrupt { month: 1 }.format_status(),
            "You are BANKRUPT in the first month"
        );
        assert_eq!(
            Status::Bankrupt { month: 24 }.format_status(),
            "In 24 months you become BANKRUPT."
        );
    }

    #[test]
    fn test_survivor_formatting() {
        assert_eq!(
            Status::Survivor { month: 1 }.format_status(),
            "You are a SURVIVOR from the first month."
        );
        assert_eq!(
            Status::Survivor { month: 60 }.format_status(),
            "In 60 months you become SURVIVOR."
        );
    }

    #[test]
    fn test_static_statuses() {
        assert_eq!(Status::InTheGame.format_status(), "You are InTheGame.");
        assert_eq!(Status::StillAlive.format_status(), "You are StillAlive.");
    }
}
