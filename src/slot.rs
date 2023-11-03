use crate::dstv_element::ParseDstvError;
use crate::get_f64_from_str;
use crate::prelude::{DstvElement, PartFace};
use std::str::FromStr;

/// Represents a slot element
/// A slot element is a hole that has been cut out of a plate but is not a circle shaped hole
pub struct Slot {
    /// Angle of the slot
    pub angle: f64,
    /// Length of the slot
    pub slot_length: f64,
    /// Width of the slot
    pub slot_width: f64,
    /// Diameter of the pub
    pub diameter: f64,
    /// Depth of the pub
    pub depth: f64,
    /// X coordinate of the pub
    pub x_coord: f64,
    /// Y coordinate of the pub
    pub y_coord: f64,
    /// Flange code of the pub
    pub fl_code: PartFace,
}

impl DstvElement for Slot {
    /// Parses a slot from a line of text
    /// # Arguments
    /// * `line` - A line of text from a DSTV file
    /// # Returns
    /// A `Result` containing either a `Slot` or an error message
    fn from_str(line: &str) -> Result<Self, ParseDstvError> {
        let mut iter = line.split_whitespace();
        let fl_code = PartFace::from_str(iter.next().unwrap()).expect("Invalid flange code");
        let x_coord = get_f64_from_str(iter.next(), "x_coord");
        let y_coord = get_f64_from_str(iter.next(), "y_coord");
        let diameter = get_f64_from_str(iter.next(), "diameter");
        let depth = get_f64_from_str(iter.next(), "depth");
        let slot_length = get_f64_from_str(iter.next(), "slot_length");
        let slot_width = get_f64_from_str(iter.next(), "slot_width");
        let angle = get_f64_from_str(iter.next(), "angle");
        Ok(Self {
            angle,
            slot_length,
            slot_width,
            diameter,
            depth,
            x_coord,
            y_coord,
            fl_code,
        })
    }

    /// Converts a slot to an SVG circle
    /// # Returns
    /// An SVG circle element
    fn to_svg(&self) -> String {
        format!(
            "<rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\" fill=\"white\" rx=\"{}\" />",
            self.x_coord,
            self.y_coord,
            self.slot_length + self.diameter,
            self.diameter,
            self.diameter / 2.0,
        )
    }

    fn get_index(&self) -> usize {
        2
    }

    fn get_facing(&self) -> &PartFace {
        &self.fl_code
    }
}
