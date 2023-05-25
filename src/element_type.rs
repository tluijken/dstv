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
}
