use crate::prelude::ParseDstvError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PartFace {
    Front,
    Top,
    Bottom,
    Behind,
}

impl FromStr for PartFace {
    type Err = ParseDstvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s {
            "v" => Ok(PartFace::Front),
            "o" => Ok(PartFace::Top),
            "u" => Ok(PartFace::Bottom),
            "h" => Ok(PartFace::Behind),
            _ => Err(ParseDstvError::new(format!("Invalid Face: {s}"))),
        }
    }
}

impl Default for PartFace {
    fn default() -> Self {
        Self::Front
    }
}
