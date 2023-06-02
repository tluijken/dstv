use crate::prelude::{DstvElement, ElementType, Header};

pub struct Dstv {
    pub header: Header,
    pub elements: Vec<Box<dyn DstvElement>>,
}

impl Dstv {
    pub fn from_file(file_path: &str) -> Result<Self, &'static str> {
        let lines = std::fs::read_to_string(file_path).expect("Unable to read file");

        let elements = lines
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
                    ElementType::None => acc.last_mut().unwrap().1.push(line.trim()),
                    ElementType::EndOfFile => {}
                    _ => acc.push((element_type, vec![])),
                }
                acc
            });

        let header = Header::from_lines(elements[0].1.clone());
        for element in &elements {
            println!("{:?}", element.0);
        }
        match header {
            Ok(_) => Ok(Self {
                header: header.unwrap(),
                elements: elements
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
                    .flatten()
                    .collect(),
            }),
            Err(_) => Err("Invalid Header"),
        }
    }

    pub fn to_svg(&self) -> String {
        format!("<svg viewbox=\"0 0 {} {}\" width=\"{}\" height=\"{}\" xmlns=\"http://www.w3.org/2000/svg\">{}</svg>",
            self.header.length,
            self.header.profile_height,
            self.header.length,
            self.header.profile_height,
            self.elements
                .iter()
                .map(|element| element.to_svg())
                .collect::<Vec<String>>()
                .join(""))
    }
}

