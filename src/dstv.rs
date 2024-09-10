use crate::{
    dstv_element::ParseDstvError,
    prelude::{DstvElement, ElementType, Header, PartFace},
};

/// Represents a DSTV file
/// Includes a header and a vector of DSTV elements
pub struct Dstv {
    /// The header of the DSTV file containing information about the project, phase, etc.
    pub header: Header,
    /// A vector of DSTV elements, e.g. cuts, holes, borders, etc.
    pub elements: Vec<Box<dyn DstvElement>>,
}

impl Dstv {
    /// Creates a new Dstv struct from a file path
    /// # Arguments
    /// * `file_path` - A string slice/path that holds the path to the file
    /// # Returns
    /// * A Result containing either a Dstv struct or a &'static str
    pub fn from_file<P: AsRef<std::path::Path>>(file_path: P) -> Result<Self, ParseDstvError> {
        let file_path = file_path.as_ref();
        let file = std::fs::read_to_string(file_path).map_err(|e| {
            ParseDstvError::new(&format!("Unable to read file: `{file_path:#?}`\t{e}"))
        })?;
        Self::from_str(file)
    }
    /// Creates a new Dstv struct from an already read file
    /// # Arguments
    /// * `file` - The file as read to String
    /// # Returns
    /// * A Result containing either a Dstv struct or a &'static str
    pub fn from_str<S: AsRef<str>>(file: S) -> Result<Self, ParseDstvError> {
        let elements = file
            .as_ref()
            .lines()
            .filter(|line| !line.trim().starts_with('*'))
            // create a tuple of (element_type, lines)
            .fold(Vec::<(ElementType, Vec<&str>)>::new(), |mut acc, line| {
                // get the element type from the first two characters
                let element_type = match &line.is_empty() {
                    true => ElementType::None,
                    false => ElementType::from_str(&line[0..2]),
                };
                match element_type {
                    ElementType::None if !acc.is_empty() => {
                        acc.last_mut().unwrap().1.push(line.trim())
                    }
                    ElementType::None => (),
                    ElementType::EndOfFile => (),
                    _ => acc.push((element_type, vec![])),
                }
                acc
            });

        let header = Header::from_lines(elements[0].1.clone())
            .map_err(|e| ParseDstvError::from_err("Invalid Header", e))?;
        let elements: Result<Vec<_>, ParseDstvError> = elements
            .iter()
            .skip(1)
            // TODO, remove this filter once all implementations are finished
            .filter(|element| {
                element.0 == ElementType::Numeration
                    || element.0 == ElementType::Bends
                    || element.0 == ElementType::Hole
                    || element.0 == ElementType::OuterBorder
                    || element.0 == ElementType::InnerBorder
                    || element.0 == ElementType::Cuts
            })
            .map(|element| element.0.parse_dstv_element(&element.1))
            // .flatten()
            .collect();
        let elements = elements?.into_iter().flatten().collect();
        Ok(Self { header, elements })
    }

    /// Converts the Dstv struct to a string of SVG
    /// # Returns
    /// * A string slice containing the SVG
    pub fn to_svg(&mut self) -> String {
        self.elements.sort_by_key(|element| element.get_index());

        // the length if the viewbox is twice the size of the number of sides > 2
        let mut svg = String::from("");

        let mut offset = 0.0;

        // Start with the bottom elements
        let bottom_elements = self
            .elements
            .iter()
            .filter(|element| element.get_facing() == &PartFace::Bottom)
            .map(|element| element.to_svg())
            .collect::<Vec<String>>()
            .join("");

        if !bottom_elements.is_empty() {
            svg.push_str(&format!(
                "<g transform=\"translate(0,{})\" id=\"bottom\">{}</g>",
                0, bottom_elements
            ));
            // Append the flange width to the offset, which sets the entry point for the front
            // elements
            offset += self.header.flange_width;
        }

        // Then the front elements
        let front_elements = self
            .elements
            .iter()
            .filter(|element| element.get_facing() == &PartFace::Front)
            .map(|element| element.to_svg())
            .collect::<Vec<String>>()
            .join("");
        if !front_elements.is_empty() {
            svg.push_str(&format!(
                "<g transform=\"translate(0,{})\" id=\"front\">{}</g>",
                offset, front_elements
            ));
            // Append the profile height to the offset, which sets the entry point for the top
            // elements
            offset += self.header.profile_height;
        }

        let top_elements = self
            .elements
            .iter()
            .filter(|element| element.get_facing() == &PartFace::Top)
            .map(|element| element.to_svg())
            .collect::<Vec<String>>()
            .join("");
        if !top_elements.is_empty() {
            // Since we're flipping this part, append the flange width to the offset, to align the
            // item well.
            offset += self.header.flange_width;
            svg.push_str(&format!(
                "<g transform=\"translate(0,{}) scale(1, -1)\" id=\"top\">{}</g>",
                offset, top_elements
            ));
        }

        // Render the back elements last
        let back_elements = self
            .elements
            .iter()
            .filter(|element| element.get_facing() == &PartFace::Behind)
            .map(|element| element.to_svg())
            .collect::<Vec<String>>()
            .join("");

        if !back_elements.is_empty() {
            svg.push_str(&format!(
                "<g transform=\"translate(0,{})\" id=\"back\">{}</g>",
                offset, back_elements
            ));
            // Append the profile height to the offset, so we know how height the SVG should be
            offset += self.header.profile_height;
        }

        format!("<svg viewbox=\"0 0 {} {}\" width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            self.header.length,
            offset,
            self.header.length,
            offset,
            svg)
    }
}
