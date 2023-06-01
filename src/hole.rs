pub use crate::prelude::DstvElement;
use crate::{get_f64_from_str, validate_flange};

pub struct Hole {
    pub diameter: f64,
    pub depth: f64,
    pub x_coord: f64,
    pub y_coord: f64,
    pub fl_code: String,
}

impl DstvElement for Hole {
    fn from_str(line: &str) -> Result<Self, &'static str> {
        let mut iter = line.split_whitespace();
        let fl_code = iter.next().unwrap();
        if !validate_flange(fl_code) {
            return Err("Invalid flange code");
        }
        let x_coord = get_f64_from_str(iter.next(), "x_coord");
        let y_coord = get_f64_from_str(iter.next(), "y_coord");
        let diameter = get_f64_from_str(iter.next(), "diameter");
        let depth = get_f64_from_str(iter.next(), "depth");
        Ok(Self {
            diameter,
            depth,
            x_coord,
            y_coord,
            fl_code: fl_code.to_string(),
        })
    }

    fn to_svg(&self) -> String {
        format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\" fill=\"white\" />",
            self.x_coord,
            self.y_coord,
            self.diameter / 2.0
        )
    }

    fn is_contour(&self) -> bool {
        false
    }
}
