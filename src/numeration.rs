pub use crate::prelude::DstvElement;
use crate::{dstv_element::ParseDstvError, get_f64_from_str, prelude::PartFace};
use std::str::FromStr;

/// Represents a numeration element
/// A numeration element is a text element that is used to label a part
pub struct Numeration {
    /// Angle of the text
    pub angle: f64,
    /// Height of the text
    pub letterheight: f64,
    /// Text to be displayed
    pub text: String,
    /// X coordinate of the text
    pub x_coord: f64,
    /// Y coordinate of the text
    pub y_coord: f64,
    /// Flange code of the text
    pub fl_code: PartFace,
}

impl DstvElement for Numeration {
    /// Parses a numeration element from a line of text
    /// # Arguments
    /// * `line` - A line of text from a DSTV file
    /// # Returns
    /// A `Result` containing either a `Numeration` or an error message
    fn from_str(line: &str) -> Result<Self, ParseDstvError> {
        let mut iter = line.split_whitespace();
        let fl_code = PartFace::from_str(iter.next().ok_or(ParseDstvError::new("No Hole Found"))?)?;
        let x_coord = get_f64_from_str(iter.next(), "x_coord")?;
        let y_coord = get_f64_from_str(iter.next(), "y_coord")?;
        let angle = get_f64_from_str(iter.next(), "angle")?;
        let letterheight = get_f64_from_str(iter.next(), "letterheight")?;
        let text = iter
            .next()
            .ok_or(ParseDstvError::new("Text element not found"))?
            .to_string();

        Ok(Self {
            angle,
            letterheight,
            text,
            x_coord,
            y_coord,
            fl_code,
        })
    }

    /// Converts a numeration element to an SVG text element
    fn to_svg(&self) -> String {
        // todo: implement
        "".to_string()
    }

    fn get_index(&self) -> usize {
        2
    }

    fn get_facing(&self) -> &PartFace {
        &self.fl_code
    }
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}
