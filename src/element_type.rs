use crate::prelude::{Bend, Cut, DstvElement, Hole, InnerBorder, Numeration, OuterBorder, Slot};

/// Enum to represent the different types of elements in a DSTV file
/// The enum is used to parse the DSTV file into a vector of DSTV elements
/// The enum is also used to determine which DSTV elements to include in the final output
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ElementType {
    /// No element type, used for empty lines or lines that are not valid DSTV element identifiers
    None,
    /// The header of the DSTV file
    Header,
    /// The outer border of the DSTV file
    OuterBorder,
    /// The inner border of the DSTV file
    InnerBorder,
    /// The powder point notes of the DSTV file
    PowderPointNotes,
    /// The punch points of the DSTV file
    PunchPoint,
    /// The holes of the DSTV file
    Hole,
    /// The numeration of the DSTV file
    Numeration,
    /// The bends of the DSTV file
    Bends,
    /// The cuts of the DSTV file
    Cuts,
    /// The end of file identifier of the DSTV file
    EndOfFile,
}

impl ElementType {
    /// Creates a new ElementType from a string slice
    /// # Arguments
    /// * `element_type` - A string slice that holds the element type
    /// # Returns
    /// * An ElementType
    pub fn from_str(element_type: &str) -> Self {
        match element_type {
            "ST" => ElementType::Header,
            "AK" => ElementType::OuterBorder,
            "IK" => ElementType::InnerBorder,
            "PU" => ElementType::PowderPointNotes,
            "KO" => ElementType::PunchPoint,
            "BO" => ElementType::Hole,
            "SI" => ElementType::Numeration,
            "KA" => ElementType::Bends,
            "SC" => ElementType::Cuts,
            "EN" => ElementType::EndOfFile,
            _ => ElementType::None,
        }
    }

    /// Parses a vector of string slices into a vector of DSTV elements
    /// # Arguments
    /// * `lines` - A vector of string slices that holds the lines of the DSTV file
    /// # Returns
    /// * A vector of DSTV elements
    pub fn parse_dstv_element(&self, lines: &Vec<&str>) -> Vec<Box<dyn DstvElement>> {
        match self {
            ElementType::OuterBorder => vec![Box::new(OuterBorder::from_lines(lines))],
            ElementType::InnerBorder => vec![Box::new(InnerBorder::from_lines(lines))],
            _ => lines
                .iter()
                .filter(|line| !line.is_empty())
                .map(|line| self.parse_dstv_element_from_line(line))
                .collect(),
        }
    }

    /// Parses a string slice into a DSTV element
    /// # Arguments
    /// * `line` - A string slice that holds the line of the DSTV file
    /// # Returns
    /// * A DSTV element
    /// # Panics
    /// * If the string slice is not a valid DSTV element
    fn parse_dstv_element_from_line(&self, line: &str) -> Box<dyn DstvElement> {
        match self {
            ElementType::Numeration => Box::new(Numeration::from_str(line).unwrap()),
            ElementType::Bends => Box::new(Bend::from_str(line).unwrap()),
            ElementType::Cuts => Box::new(Cut::from_str(line).unwrap()),
            ElementType::Hole => match &line.split_whitespace().count() < &8 {
                true => Box::new(Hole::from_str(line).unwrap()),
                false => Box::new(Slot::from_str(line).unwrap()),
            },
            _ => panic!("Invalid Element Type"),
        }
    }
}
