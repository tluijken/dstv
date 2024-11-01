use crate::{
    dstv_element::ParseDstvError,
    dstv_element_type::DstvElementType,
    prelude::{
        Bend, Cut, DstvElement, Header, Hole, InnerBorder, Numeration, OuterBorder, PartFace, Slot,
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

const HOLE_TYPE: &str = "BO";
const CUT_TYPE: &str = "SC";
const BEND_TYPE: &str = "KA";
const OUTER_BORDER_TYPE: &str = "AK";
const INNER_BORDER_TYPE: &str = "IK";
const NUMERATION_TYPE: &str = "SI";
const START: &str = "ST";
const END: &str = "EN";

/// Helper function to parse a line into a specific `DstvElementType` variant
fn parse_dstv_element(
    element_type: &str,
    lines: &Vec<&str>,
) -> Result<DstvElementType, ParseDstvError> {
    match element_type {
        OUTER_BORDER_TYPE => OuterBorder::from_lines(lines).map(DstvElementType::OuterBorder),
        INNER_BORDER_TYPE => InnerBorder::from_lines(lines).map(DstvElementType::InnerBorder),
        _ => match element_type {
            CUT_TYPE => Cut::from_str(lines[0]).map(DstvElementType::Cut),
            BEND_TYPE => Bend::from_str(lines[0]).map(DstvElementType::Bend),
            HOLE_TYPE => match lines[0].split_whitespace().count() > 7 {
                true => Slot::from_str(lines[0]).map(DstvElementType::Slot),
                false => Hole::from_str(lines[0]).map(DstvElementType::Hole),
            },
            NUMERATION_TYPE => Numeration::from_str(lines[0]).map(DstvElementType::Numeration),
            _ => Err(ParseDstvError::new(&format!(
                "Unknown element type: `{}`",
                element_type
            ))),
        },
    }
}

impl Dstv {
    pub fn from_file<P: AsRef<std::path::Path>>(file_path: P) -> Result<Self, ParseDstvError> {
        let file_path = file_path.as_ref();
        let file = std::fs::read_to_string(file_path).map_err(|e| {
            ParseDstvError::new(&format!("Unable to read file: `{file_path:#?}`\t{e}"))
        })?;
        Self::from_str(&file)
    }

    pub fn from_str<S: AsRef<str>>(file: S) -> Result<Self, ParseDstvError> {
        let file_content = file.as_ref();
        let lines = file_content
            .lines()
            .filter(|line| !line.trim().starts_with('*'));

        let header_lines = extract_header_lines(&mut lines.clone());
        let header = Header::from_lines(header_lines.clone())
            .map_err(|e| ParseDstvError::from_err("Invalid Header", e))?;

        let element_groups = group_elements_by_type(lines.skip(header_lines.len()).into_iter());

        let elements = parse_elements(element_groups)?;
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
                    format!(
                        "translate(0,{}) scale(1, -1)",
                        offset + self.header.flange_width
                    )
                } else {
                    format!("translate(0,{})", offset)
                };

                svg.push_str(&format!(
                    "<g transform=\"{}\" id=\"{}\">{}</g>",
                    transform, id, elements_svg
                ));

                offset += if *face == PartFace::Front || *face == PartFace::Behind {
                    self.header.profile_height
                } else {
                    self.header.flange_width
                };
            }
        }

        format!(
            "<svg viewbox=\"0 0 {} {}\" width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            self.header.length, offset, self.header.length, offset, svg
        )
    }
}

/// Extracts header lines from the beginning of the file iterator `lines`.
/// Stops when it encounters the first empty line following non-empty lines.
fn extract_header_lines<'a, I>(lines: &mut I) -> Vec<&'a str>
where
    I: Iterator<Item = &'a str> + Clone,
{
    let mut empty_lines_found = false;
    lines
        .clone()
        .take_while(|line| {
            if empty_lines_found {
                return line.trim().is_empty();
            }
            empty_lines_found = line.trim().is_empty();
            true
        })
        .filter(|line| !line.starts_with(START))
        .map(|line| line.trim())
        .collect()
}

/// Groups lines into (element type, lines) pairs based on the first two characters.
/// Stops processing when it encounters the `END` line.
fn group_elements_by_type<'a, I>(lines: I) -> Vec<(&'a str, Vec<&'a str>)>
where
    I: Iterator<Item = &'a str>,
{
    lines
        .into_iter()
        .filter(|line| !line.eq(&END))
        .filter(|line| !line.trim().is_empty())
        .fold(Vec::<(&str, Vec<&str>)>::new(), |mut elements, line| {
            // check if the first two characters of the line are empty
            let current_element = elements.last();
            match line[..2].trim().is_empty() {
                true => {
                    if current_element.is_some()
                        // chef special for holes, which have a single element indicating the hole type
                        // followed by multiple hole lines
                        && current_element.unwrap().0.eq(HOLE_TYPE)
                        && current_element.unwrap().1.len() == 1
                    {
                        elements.push((HOLE_TYPE, vec![line]));
                    } else {
                        elements.last_mut().unwrap().1.push(line);
                    }
                }
                false => {
                    elements.push((line, Vec::new()));
                }
            }
            elements
        })
        .into_iter()
        .collect::<Vec<_>>()
}

/// Parses each element group into `DstvElementType`.
fn parse_elements(
    element_groups: Vec<(&str, Vec<&str>)>,
) -> Result<Vec<DstvElementType>, ParseDstvError> {
    element_groups
        .into_iter()
        .map(|(element_type, lines)| parse_dstv_element(element_type, &lines))
        .filter(|element| element.is_ok())
        .collect::<Result<Vec<_>, _>>()
}
