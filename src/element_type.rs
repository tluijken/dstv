use crate::prelude::{Bend, Cut, DstvElement, Hole, InnerBorder, Numeration, OuterBorder, Slot};

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
