use crate::prelude::{DstvElement, ElementType, Header};
use std::collections::HashMap;

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
        match header {
            Ok(_) => Ok(Self {
                header: header.unwrap(),
                elements: vec![],
            }),
            Err(_) => Err("Invalid Header"),
        }
    }

    pub fn to_svg() -> String {
        todo!()
    }
}

