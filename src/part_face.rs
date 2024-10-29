use crate::prelude::ParseDstvError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PartFace {
    /// The Front or `v` Face
    Front,
    /// The Top or `o` Face
    Top,
    /// The Bottom or `u` Face
    Bottom,
    /// The Behind or `h` Face
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
