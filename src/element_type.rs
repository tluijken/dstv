use crate::prelude::{Bend, DstvElement, Hole, Numeration, Slot};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ElementType {
    None,
    Header,
    OuterBorder,
    InnerBorder,
    PowderPointNotes,
    PunchPoint,
    Hole,
    Numeration,
    Bends,
    Cuts,
    EndOfFile,
}

impl ElementType {
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

    pub fn parse_dstv_element(&self, lines: &Vec<&str>) -> Vec<Box<dyn DstvElement>> {
        lines
            .iter()
            .map(|line| self.parse_dstv_element_from_line(line))
            .collect()
    }

    fn parse_dstv_element_from_line(&self, line: &str) -> Box<dyn DstvElement> {
        match self {
            ElementType::Numeration => Box::new(Numeration::from_lines(line).unwrap()),
            ElementType::Bends => Box::new(Bend::from_lines(line).unwrap()),
            ElementType::Hole => match &line.split_whitespace().count() {
                4 => Box::new(Hole::from_lines(line).unwrap()),
                5 => Box::new(Hole::from_lines(line).unwrap()),
                8 => Box::new(Slot::from_lines(line).unwrap()),
                _ => panic!("Invalid Hole"),
            },
            _ => panic!("Invalid Element Type"),
        }
    }
}
