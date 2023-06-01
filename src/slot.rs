use crate::prelude::DstvElement;
use crate::validate_flange;

pub struct Slot {
    pub angle: f64,
    pub slot_length: f64,
    pub slot_width: f64,
    pub diameter: f64,
    pub depth: f64,
    pub x_coord: f64,
    pub y_coord: f64,
    pub fl_code: String,
}

impl DstvElement for Slot {
    fn from_lines(line: &str) -> Result<Self, &'static str> {
        let mut iter = line.split_whitespace();
        let fl_code = iter.next().unwrap();
        if !validate_flange(fl_code) {
            return Err("Invalid flange code");
        }
        let x_coord = iter
            .next()
            .unwrap()
            .replace("s", "")
            .parse::<f64>()
            .unwrap();
        let y_coord = iter.next().unwrap().parse::<f64>().unwrap();
        let angle = iter.next().unwrap().parse::<f64>().unwrap();
        let slot_length = iter.next().unwrap().parse::<f64>().unwrap();
        let slot_width = iter.next().unwrap().parse::<f64>().unwrap();
        let diameter = iter.next().unwrap().parse::<f64>().unwrap();
        let depth = iter.next().unwrap().parse::<f64>().unwrap();
        Ok(Self {
            angle,
            slot_length,
            slot_width,
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
