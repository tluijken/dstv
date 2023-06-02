use crate::{dstv_element::DstvElement, get_f64_from_str, validate_flange};

pub struct OuterBorder {
    pub contour: Vec<BorderPoint>,
}

pub struct InnerBorder {
    pub contour: Vec<BorderPoint>,
}

#[derive(Clone, Debug, Default)]
pub struct BorderPoint {
    pub fl_code: String,
    pub x_coord: f64,
    pub y_coord: f64,
    pub radius: f64,
}

fn read_contour(lines: &[&str]) -> Vec<BorderPoint> {
    lines
        .iter()
        .map(|line| {
            let mut iter = line.split_whitespace();
            let fl_code = iter
                .next()
                .map(|s| if validate_flange(s) { s } else { "x" })
                .unwrap_or("x");

            let x_coord = get_f64_from_str(iter.next(), "x_coord");
            let y_coord = get_f64_from_str(iter.next(), "y_coord");
            let radius = get_f64_from_str(iter.next(), "radius");

            BorderPoint {
                fl_code: fl_code.to_string(),
                x_coord,
                y_coord,
                radius,
            }
        })
        .collect()
}

fn get_bend(point: &BorderPoint, prev: &BorderPoint) -> (f64, f64, f64, f64) {
    match (prev.y_coord > point.y_coord, point.x_coord > prev.x_coord) {
        (true, true) => (prev.x_coord, point.y_coord, point.x_coord, point.y_coord), // left-top corner
        (false, true) => (point.x_coord, prev.y_coord, point.x_coord, point.y_coord), // top-right corner
        (false, false) => (prev.x_coord, point.y_coord, point.x_coord, point.y_coord), // right-bottom corner
        (true, false) => (point.x_coord, prev.y_coord, point.x_coord, point.y_coord), // bottom-left corner
    }
}

fn contour_to_svg(contour: &Vec<BorderPoint>, color: &str) -> String {
    let (path_str, _) = contour.iter().enumerate().fold(
        (String::new(), BorderPoint::default()),
        |(mut path, prev), (i, point)| {
            let segment = if i == 0 {
                format!("M{},{} ", point.x_coord - point.radius, point.y_coord)
            } else if prev.radius > 0.0 {
                let (x1, y1, x2, y2) = get_bend(point, &prev);
                format!("Q{},{},{},{} ", x1, y1, x2, y2)
            } else {
                format!("L{},{} ", point.x_coord, point.y_coord)
            };
            path.push_str(&segment);
            (path, point.clone())
        },
    );

    format!(
        "<path d=\"{}\" fill=\"{}\" stroke=\"black\" stroke-width=\"0.5\" />",
        path_str, color
    )
}

impl OuterBorder {
    pub fn from_lines(lines: &[&str]) -> Self {
        Self {
            contour: read_contour(lines),
        }
    }
}

impl InnerBorder {
    pub fn from_lines(lines: &[&str]) -> Self {
        Self {
            contour: read_contour(lines),
        }
    }
}
impl DstvElement for OuterBorder {
    fn to_svg(&self) -> String {
        contour_to_svg(&self.contour, "grey")
    }

    fn from_str(_line: &str) -> Result<Self, &'static str> {
        todo!("Find out how to split traits and casts when when calling in a idiomatic way");
    }
}

impl DstvElement for InnerBorder {
    fn to_svg(&self) -> String {
        contour_to_svg(&self.contour, "white")
    }

    fn from_str(_line: &str) -> Result<Self, &'static str> {
        todo!("Find out how to split traits and casts when when calling in a idiomatic way");
    }
}
