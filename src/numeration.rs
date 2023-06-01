pub use crate::prelude::DstvElement;
use crate::{get_f64_from_str, validate_flange};

pub struct Numeration {
    pub angle: f64,
    pub letterheight: f64,
    pub text: String,
    pub x_coord: f64,
    pub y_coord: f64,
    pub fl_code: String,
}

impl DstvElement for Numeration {
    fn from_str(line: &str) -> Result<Self, &'static str> {
        let mut iter = line.split_whitespace();
        let fl_code = iter.next().unwrap();
        let flange = match validate_flange(fl_code) {
            true => fl_code,
            false => "x",
        };

        let x_coord = get_f64_from_str(iter.next(), "x_coord");
        let y_coord = get_f64_from_str(iter.next(), "y_coord");
        let angle = get_f64_from_str(iter.next(), "angle");
        let letterheight = get_f64_from_str(iter.next(), "letterheight");
        let text = iter.next().expect("Text element not found").to_string();

        Ok(Self {
            angle,
            letterheight,
            text,
            x_coord,
            y_coord,
            fl_code: flange.to_string(),
        })
    }

    fn to_svg(&self) -> String {
        // todo: implement
        "".to_string()
    }
}
