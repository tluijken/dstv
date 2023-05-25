use crate::prelude::{DstvElement, ElementType, Header};
use std::collections::HashMap;

pub struct Dstv {
    pub header: Header,
    pub elements: Vec<Box<dyn DstvElement>>,
}

impl Dstv {
    pub fn from_file(file_path: &str) -> Self {
        let lines = std::fs::read_to_string(file_path).expect("Unable to read file");

        let elements = lines
            .lines()
            .filter(|line| !line.is_empty())
            .filter(|line| !line.trim().starts_with('*'))
            // create a tuple of (element_type, lines)
            .fold(Vec::<(ElementType, Vec<&str>)>::new(), |mut acc, line| {
                // get the element type from the first two characters
                let element_type = ElementType::from_str(&line[0..2]);
                if element_type != ElementType::None && element_type != ElementType::EndOfFile {
                    // if the first two characters represent an element type, start a new element
                    // block
                    acc.push((element_type, Vec::<&str>::new()));
                } else {
                    // if the first two characters are not a valid element type, then it is a continuation of the previous element
                    acc.last_mut().unwrap().1.push(line);
                }
                acc
            });
        println!("{:?}", elements);
        todo!()
    }

    pub fn to_svg() -> String {
        todo!()
    }
}

