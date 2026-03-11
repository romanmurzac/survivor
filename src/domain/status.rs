#[derive(Debug)]
pub enum Status {
    Bankrupt { month: u32 },
    StillAlive,
    InTheGame,
    Survivor { month: u32 },
}

impl Status {
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
