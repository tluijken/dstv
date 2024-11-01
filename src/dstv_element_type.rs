use crate::prelude::{
    Bend, Cut, DstvElement, Hole, InnerBorder, Numeration, OuterBorder, PartFace, Slot,
};

#[derive(Debug)]
pub enum DstvElementType {
    OuterBorder(OuterBorder),
    InnerBorder(InnerBorder),
    Cut(Cut),
    Bend(Bend),
    Slot(Slot),
    Hole(Hole),
    Numeration(Numeration),
}

impl DstvElementType {
    /// Returns the SVG representation of each element based on type
    pub fn to_svg(&self) -> String {
        match self {
            DstvElementType::OuterBorder(e) => e.to_svg(),
            DstvElementType::InnerBorder(e) => e.to_svg(),
            DstvElementType::Cut(e) => e.to_svg(),
            DstvElementType::Bend(e) => e.to_svg(),
            DstvElementType::Slot(e) => e.to_svg(),
            DstvElementType::Hole(e) => e.to_svg(),
            DstvElementType::Numeration(e) => e.to_svg(),
        }
    }

    /// Returns the index used to determine the rendering order
    pub fn get_index(&self) -> usize {
        match self {
            DstvElementType::OuterBorder(e) => e.get_index(),
            DstvElementType::InnerBorder(e) => e.get_index(),
            DstvElementType::Cut(e) => e.get_index(),
            DstvElementType::Bend(e) => e.get_index(),
            DstvElementType::Slot(e) => e.get_index(),
            DstvElementType::Hole(e) => e.get_index(),
            DstvElementType::Numeration(e) => e.get_index(),
        }
    }

    /// Returns the face direction of each element
    pub fn get_facing(&self) -> &PartFace {
        match self {
            DstvElementType::OuterBorder(e) => e.get_facing(),
            DstvElementType::InnerBorder(e) => e.get_facing(),
            DstvElementType::Cut(e) => e.get_facing(),
            DstvElementType::Bend(e) => e.get_facing(),
            DstvElementType::Slot(e) => e.get_facing(),
            DstvElementType::Hole(e) => e.get_facing(),
            DstvElementType::Numeration(e) => e.get_facing(),
        }
    }
}
