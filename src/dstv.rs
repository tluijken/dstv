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
                match element_type {
                    ElementType::None => acc.last_mut().unwrap().1.push(line),
                    ElementType::EndOfFile => {}
                    _ => acc.push((element_type, vec![line])),
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

