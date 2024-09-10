pub use crate::prelude::DstvElement;
use crate::{dstv_element::ParseDstvError, get_f64_from_str, prelude::PartFace};
use std::str::FromStr;
/// Represents a hole in a plate
pub struct Hole {
    /// Diameter of the hole
    pub diameter: f64,
    /// Depth of the hole
    pub depth: f64,
    /// X coordinate of the hole
    pub x_coord: f64,
    /// Y coordinate of the hole
    pub y_coord: f64,
    /// Flange code of the hole
    pub fl_code: PartFace,
}

impl DstvElement for Hole {
    /// Parses a hole from a line of text
    /// # Arguments
    /// * `line` - A line of text from a DSTV file
    /// # Returns
    /// A `Result` containing either a `Hole` or an error message
    fn from_str(line: &str) -> Result<Self, ParseDstvError> {
        let mut iter = line.split_whitespace();
        let fl_code = PartFace::from_str(iter.next().ok_or(ParseDstvError::new("No Hole Found"))?)?;
        let x_coord = get_f64_from_str(iter.next(), "x_coord")?;
        let y_coord = get_f64_from_str(iter.next(), "y_coord")?;
        let diameter = get_f64_from_str(iter.next(), "diameter")?;
        let depth = get_f64_from_str(iter.next(), "depth")?;
        Ok(Self {
            diameter,
            depth,
            x_coord,
            y_coord,
            fl_code,
        })
    }

    /// Converts a hole to an SVG circle
    /// # Returns
    /// An SVG circle element
    fn to_svg(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"white\" />",
            self.x_coord,
            self.y_coord,
            self.diameter / 2.0
        )
    }

    fn get_index(&self) -> usize {
        2
    }

    fn get_facing(&self) -> &PartFace {
        &self.fl_code
    }
}
