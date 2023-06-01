pub use crate::prelude::DstvElement;
use crate::validate_flange;

pub struct Numeration {
    pub angle: f64,
    pub letterheight: f64,
    pub text: String,
    pub x_coord: f64,
    pub y_coord: f64,
    pub fl_code: String,
}

impl DstvElement for Numeration {
    fn from_lines(line: &str) -> Result<Self, &'static str> {
        let mut iter = line.split_whitespace();
        let fl_code = iter.next().unwrap();
        let flange = match validate_flange(fl_code) {
            true => fl_code,
            false => "x",
        };

        let x_coord = iter
            .next()
            .unwrap()
            .replace("o", "")
            .parse::<f64>()
            .unwrap();
        let y_coord = iter.next().unwrap().parse::<f64>().unwrap();
        let angle = iter.next().unwrap().parse::<f64>().unwrap();
        let letterheight = iter.next().unwrap().parse::<f64>().unwrap();
        let text = iter.next().unwrap().to_string();
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
        let mut svg = String::new();
        svg.push_str(&format!(
            "<text x=\"{}\" y=\"{}\" transform=\"rotate({} {} {})\" font-size=\"{}\">{}</text>",
            self.x_coord,
            self.y_coord,
            self.angle,
            self.x_coord,
            self.y_coord,
            self.letterheight,
            self.text
        ));
        svg
    }

    fn is_contour(&self) -> bool {
        false
    }
}
