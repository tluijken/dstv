use crate::get_f64_from_str;
use crate::prelude::DstvElement;

pub struct Bend {
    pub angle: f64,
    pub radius: f64,
    pub finish_x: f64,
    pub finish_y: f64,
    pub origin_x: f64,
    pub origin_y: f64,
}

impl DstvElement for Bend {
    fn from_str(line: &str) -> Result<Self, &'static str> {
        let mut iter = line.split_whitespace();
        let origin_x = get_f64_from_str(iter.next(), "origin_x");
        let origin_y = get_f64_from_str(iter.next(), "origin_y");
        let angle = get_f64_from_str(iter.next(), "angle");
        let radius = get_f64_from_str(iter.next(), "radius");
        let finish_x = get_f64_from_str(iter.next(), "finish_x");
        let finish_y = get_f64_from_str(iter.next(), "finish_y");
        Ok(Self {
            angle,
            radius,
            finish_x,
            finish_y,
            origin_x,
            origin_y,
        })
    }

    fn to_svg(&self) -> String {
        format!(
            "<path d=\"M{},{} A{},{},0,0,1,{},{}\" stroke=\"black\" fill=\"none\" />",
            self.origin_x, self.origin_y, self.radius, self.radius, self.finish_x, self.finish_y
        )
    }
}
