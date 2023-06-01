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
    fn from_lines(line: &str) -> Result<Self, &'static str> {
        let mut iter = line.split_whitespace();
        let origin_x = iter.next().unwrap().parse::<f64>().unwrap();
        let origin_y = iter.next().unwrap().parse::<f64>().unwrap();
        let finish_x = iter.next().unwrap().parse::<f64>().unwrap();
        let finish_y = iter.next().unwrap().parse::<f64>().unwrap();
        let angle = iter.next().unwrap().parse::<f64>().unwrap();
        let radius = iter.next().unwrap().parse::<f64>().unwrap();
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
        let mut svg = String::new();
        svg.push_str(&format!(
            "<path d=\"M {} {} A {} {} 0 0 1 {} {}\" stroke=\"black\" fill=\"none\" />",
            self.origin_x, self.origin_y, self.radius, self.radius, self.finish_x, self.finish_y
        ));
        svg
    }

    fn is_contour(&self) -> bool {
        false
    }
}
