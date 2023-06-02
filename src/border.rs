use crate::{dstv_element::DstvElement, get_f64_from_str, validate_flange};

pub struct OuterBorder {
    pub contour: Vec<BorderPoint>,
}

pub struct InnerBorder {
    pub contour: Vec<BorderPoint>,
}

#[derive(Clone, Debug)]
pub struct BorderPoint {
    pub fl_code: String,
    pub x_coord: f64,
    pub y_coord: f64,
    pub radius: f64,
}

fn read_contour(lines: &[&str]) -> Vec<BorderPoint> {
    let mut contour = Vec::new();
    for line in lines {
        let mut iter = line.split_whitespace();
        let fl_code = match validate_flange(&iter.clone().next().unwrap_or("")) {
            true => iter.next().unwrap(),
            false => "x",
        };

        let x_coord = get_f64_from_str(iter.next(), "x_coord");
        let y_coord = get_f64_from_str(iter.next(), "y_coord");
        let radius = get_f64_from_str(iter.next(), "radius");
        println!("{} {} {}", x_coord, y_coord, radius);
        contour.push(BorderPoint {
            fl_code: fl_code.to_string(),
            x_coord,
            y_coord,
            radius,
        });
    }
    contour
}

fn contour_to_svg(contour: &Vec<BorderPoint>, color: &str) -> String {
    let mut sb = String::new();
    let mut previous = BorderPoint {
        fl_code: "x".to_string(),
        x_coord: 0.0,
        y_coord: 0.0,
        radius: 0.0,
    };
    for (i, point) in contour.iter().enumerate() {
        if i == 0 {
            sb.push_str(&format!(
                "M{},{} ",
                point.x_coord - point.radius,
                point.y_coord
            ));
        } else {
            let radius = previous.radius;
            match radius > 0.0 {
                true => {
                    if previous.y_coord > point.y_coord && point.x_coord > previous.x_coord {
                        // left-top corner
                        sb.push_str(&format!(
                            "Q{},{},{},{} ",
                            previous.x_coord, point.y_coord, point.x_coord, point.y_coord
                        ));
                    } else if previous.y_coord < point.y_coord && point.x_coord > previous.x_coord {
                        // top-right corner
                        sb.push_str(&format!(
                            "Q{},{},{},{} ",
                            point.x_coord, previous.y_coord, point.x_coord, point.y_coord
                        ));
                    } else if previous.y_coord < point.y_coord && point.x_coord < previous.x_coord {
                        // right-bottom corner
                        sb.push_str(&format!(
                            "Q{},{},{},{} ",
                            previous.x_coord, point.y_coord, point.x_coord, point.y_coord
                        ));
                    } else if previous.y_coord > point.y_coord && point.x_coord < previous.x_coord {
                        // bottom-left corner
                        sb.push_str(&format!(
                            "Q{},{},{},{} ",
                            point.x_coord, previous.y_coord, point.x_coord, point.y_coord
                        ));
                    }
                }
                false => {
                    sb.push_str(&format!("L{},{} ", point.x_coord, point.y_coord));
                }
            }
        }
        previous = point.clone();
    }

    format!(
        "<path d=\"{}\" fill=\"{}\" stroke=\"black\" stroke-width=\"0.5\" />",
        sb, color
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
