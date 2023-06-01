use crate::{dstv_element::DstvElement, prelude::ElementType};

pub struct Contour {
    pub contour_type: ElementType,
    pub contour: Vec<ContourPoint>,
}

pub struct ContourPoint {
    pub fl_code: String,
    pub x_coord: f64,
    pub y_coord: f64,
    pub radius: f64,
}

impl DstvElement for Contour {
    fn from_str(_line: &str) -> Result<Self, &'static str> {
        unimplemented!()
    }

    fn to_svg(&self) -> String {
        let mut svg = String::new();
        svg.push_str("<path d=\"");
        for (i, point) in self.contour.iter().enumerate() {
            if i == 0 {
                svg.push_str(&format!(
                    "M {} {} ",
                    point.x_coord - point.radius,
                    point.y_coord
                ));
            } else {
                svg.push_str(&format!(
                    "A {} {} 0 0 1 {} {} ",
                    point.radius,
                    point.radius,
                    point.x_coord - point.radius,
                    point.y_coord
                ));
            }
        }
        svg.push_str("\" fill=\"none\" stroke=\"black\" stroke-width=\"1\" />");
        svg
    }

    fn is_contour(&self) -> bool {
        true
    }
}
