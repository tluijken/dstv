use crate::{
    dstv_element::ParseDstvError,
    prelude::{
        DstvElement, 
        Header,
        Numeration,
        PartFace,
        Slot,
        Hole,
        Cut,
        Bend,
        InnerBorder,
        OuterBorder
    }, 
};

/// Represents a DSTV file
/// Includes a header and a vector of DSTV elements
pub struct Dstv {
    /// The header of the DSTV file containing information about the project, phase, etc.
    pub header: Header,
    /// A vector of DSTV elements, e.g. cuts, holes, borders, etc.
    pub elements: Vec<DstvElementType>,
}

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
    fn to_svg(&self) -> String {
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
    fn get_index(&self) -> usize {
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
    fn get_facing(&self) -> &PartFace {
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

/// Helper function to parse a line into a specific `DstvElementType` variant
fn parse_dstv_element(line: &str) -> Result<DstvElementType, ParseDstvError> {
    match line.get(0..2) {
        Some("OB") => OuterBorder::from_str(line).map(DstvElementType::OuterBorder),
        Some("IB") => InnerBorder::from_str(line).map(DstvElementType::InnerBorder),
        Some("CU") => Cut::from_str(line).map(DstvElementType::Cut),
        Some("BE") => Bend::from_str(line).map(DstvElementType::Bend),
        Some("SL") => Slot::from_str(line).map(DstvElementType::Slot),
        Some("HO") => Hole::from_str(line).map(DstvElementType::Hole),
        Some("NU") => Numeration::from_str(line).map(DstvElementType::Numeration),
        _ => Err(ParseDstvError::new("Unknown element type")),
    }
}

impl Dstv {
    pub fn from_file<P: AsRef<std::path::Path>>(file_path: P) -> Result<Self, ParseDstvError> {
        let file_path = file_path.as_ref();
        let file = std::fs::read_to_string(file_path)
            .map_err(|e| ParseDstvError::new(&format!("Unable to read file: `{file_path:#?}`\t{e}")))?;
        Self::from_str(&file)
    }

    pub fn from_str<S: AsRef<str>>(file: S) -> Result<Self, ParseDstvError> {
        let file_content = file.as_ref();
        let lines = file_content.lines().filter(|line| !line.trim().starts_with('*'));
        
        // Parse header from the initial lines
        let header_lines: Vec<_> = lines.clone().take_while(|line| !line.is_empty()).collect();
        let header = Header::from_lines(header_lines)
            .map_err(|e| ParseDstvError::from_err("Invalid Header", e))?;

        // Parse each element line into `DstvElementType` and collect
        let elements = lines
            .skip_while(|line| !line.is_empty()) // Skip past the header section
            .filter(|line| !line.trim().is_empty()) // Ignore empty lines
            .map(parse_dstv_element)
            .collect::<Result<Vec<DstvElementType>, ParseDstvError>>()?;

        Ok(Self { header, elements })
    }

    pub fn to_svg(&mut self) -> String {
        self.elements.sort_by_key(|element| element.get_index());
        let mut svg = String::new();
        let mut offset = 0.0;

        for (face, id) in &[
            (PartFace::Bottom, "bottom"),
            (PartFace::Front, "front"),
            (PartFace::Top, "top"),
            (PartFace::Behind, "back"),
        ] {
            let elements_svg = self
                .elements
                .iter()
                .filter(|element| element.get_facing() == face)
                .map(|element| element.to_svg())
                .collect::<Vec<_>>()
                .join("");

            if !elements_svg.is_empty() {
                let transform = if *face == PartFace::Top {
                    format!("translate(0,{}) scale(1, -1)", offset + self.header.flange_width)
                } else {
                    format!("translate(0,{})", offset)
                };

                svg.push_str(&format!("<g transform=\"{}\" id=\"{}\">{}</g>", transform, id, elements_svg));

                offset += if *face == PartFace::Front || *face == PartFace::Behind {
                    self.header.profile_height
                } else {
                    self.header.flange_width
                };
            }
        }

        format!(
            "<svg viewBox=\"0 0 {} {}\" width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            self.header.length, offset, self.header.length, offset, svg
        )
    }
}
