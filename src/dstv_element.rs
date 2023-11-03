use crate::prelude::PartFace;

#[derive(Debug)]
pub struct ParseDstvError {
    message: String,
}

impl ParseDstvError {
    pub fn new(message: &str) -> ParseDstvError {
        ParseDstvError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ParseDstvError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Could not parse DSTV element: {:?}", self.message)
    }
}

impl std::error::Error for ParseDstvError {}

pub trait DstvElement {
    /// Creates a new DSTV element from a string slice.
    /// # Arguments
    /// * `line` - A string slice that holds the line of the DSTV file
    /// # Returns
    /// * A DSTV element
    fn from_str(line: &str) -> Result<Self, ParseDstvError>
    where
        Self: Sized;

    /// Convert the element to an SVG representation.
    /// # Returns
    /// A string containing an SVG representation of the element.
    fn to_svg(&self) -> String;

    /// Returns the index of the element in the DSTV file.
    /// This is used to determine the order in which the elements are drawn.
    /// The lower the index, the earlier the element is drawn.
    fn get_index(&self) -> usize {
        0
    }

    /// Returns the flange code of the element.
    /// This is used to determine the side of the element that is drawn.
    fn get_facing(&self) -> &PartFace;
}
