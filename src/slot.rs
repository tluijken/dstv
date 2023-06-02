use crate::prelude::DstvElement;
use crate::{get_f64_from_str, validate_flange};

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
    pub fl_code: String,
}

impl DstvElement for Slot {
    /// Parses a slot from a line of text
    /// # Arguments
    /// * `line` - A line of text from a DSTV file
    /// # Returns
    /// A `Result` containing either a `Slot` or an error message
    fn from_str(line: &str) -> Result<Self, &'static str> {
        let mut iter = line.split_whitespace();
        let fl_code = iter.next().unwrap();
        if !validate_flange(fl_code) {
            return Err("Invalid flange code");
        }
        let x_coord = get_f64_from_str(iter.next(), "x_coord");
        let y_coord = get_f64_from_str(iter.next(), "y_coord");
        let angle = get_f64_from_str(iter.next(), "angle");
        let slot_length = get_f64_from_str(iter.next(), "slot_length");
        let slot_width = get_f64_from_str(iter.next(), "slot_width");
        let diameter = get_f64_from_str(iter.next(), "diameter");
        let depth = get_f64_from_str(iter.next(), "depth");
        Ok(Self {
            angle,
            slot_length,
            slot_width,
            diameter,
            depth,
            x_coord,
            y_coord,
            fl_code: fl_code.to_string(),
        })
    }

    /// Converts a slot to an SVG circle
    /// # Returns
    /// An SVG circle element
    fn to_svg(&self) -> String {
        // TODO! this should not be a circle
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"white\" />",
            self.x_coord,
            self.y_coord,
            self.diameter / 2.0
        )
    }
}
