use crate::{dstv_element::DstvElement, get_f64_from_str, validate_flange};

/// A struct representing the outer border of a DSTV file
/// A DSTV file can have multiple outer borders
pub struct OuterBorder {
    /// A vector of border points, representing the contour of the outer border
    pub contour: Vec<BorderPoint>,
}

/// A struct representing the inner border of a DSTV file
/// A DSTV file can have multiple inner borders
pub struct InnerBorder {
    /// A vector of border points, representing the contour of the inner border
    pub contour: Vec<BorderPoint>,
}

/// A struct representing a border point
/// A border point is a point on the contour of a border
/// It has an x and y coordinate and a radius
#[derive(Clone, Debug, Default)]
pub struct BorderPoint {
    /// The flange code of the border point
    pub fl_code: String,
    /// The x coordinate of the border point
    pub x_coord: f64,
    /// The y coordinate of the border point
    pub y_coord: f64,
    /// The radius of the border point
    pub radius: f64,

    pub bevel: f64,
}

/// Reads the contour of a border from a DSTV file.
/// The contour is represented by a vector of BorderPoints
/// # Arguments
/// * `lines` - A vector of strings, representing the lines of the DSTV file
/// # Returns
/// A vector of BorderPoints, representing the contour of the border
/// # Panics
/// Panics if the flange code of a border point is invalid
/// Panics if the x coordinate of a border point is invalid
/// Panics if the y coordinate of a border point is invalid
/// Panics if the radius of a border point is invalid
fn read_contour(lines: &[&str]) -> Vec<BorderPoint> {
    lines
        .iter()
        .map(|line| {
            let mut iter = line.split_whitespace().peekable();
            let first = iter.peek();
            let fl_code = match validate_flange(first.unwrap_or(&"x")) {
                true => iter.next().unwrap(),
                false => "x",
            };

            let x_coord = get_f64_from_str(iter.next(), "x_coord");
            let y_coord = get_f64_from_str(iter.next(), "y_coord");
            let radius = get_f64_from_str(iter.next(), "radius");
            let bevel = match iter.peek().is_some() {
                true => get_f64_from_str(iter.next(), "bevel"),
                false => 0.0,
            };

            BorderPoint {
                fl_code: fl_code.to_string(),
                x_coord,
                y_coord,
                radius,
                bevel,
            }
        })
        .collect()
}

/// Converts a contour to an SVG path
/// # Arguments
/// * `contour` - A vector of BorderPoints, representing the contour of a border
/// * `color` - A string representing the color of the border
/// # Returns
/// A string representing the SVG path of the border
fn contour_to_svg(contour: &Vec<BorderPoint>, color: &str) -> String {
    let mut bevel_lines = Vec::new();
    let (path_str, _) = contour.iter().enumerate().fold(
        (String::new(), BorderPoint::default()),
        |(mut path, prev), (i, point)| {
            let segment = if i == 0 {
                format!("M {} {}", point.x_coord, point.y_coord)
            } 
            else {
                match prev.radius {
                    r if r > 0.0 && prev.y_coord < point.y_coord && point.x_coord > prev.x_coord => {
                        format!(" Q {} {} {} {}", point.x_coord, prev.y_coord, point.x_coord, point.y_coord)
                    },
                    r if r > 0.0 && prev.y_coord < point.y_coord && point.x_coord < prev.x_coord => {
                        format!(" Q {} {} {} {}", prev.x_coord, point.y_coord, point.x_coord, point.y_coord)
                    },
                    r if r > 0.0 && prev.y_coord > point.y_coord && point.x_coord < prev.x_coord => {
                        format!(" Q {} {} {} {}", point.x_coord, prev.y_coord, point.x_coord, point.y_coord)
                    },
                    r if r > 0.0 || r < 0.0 => {
                        format!(" A {} {} 0 0 0 {} {}", -prev.radius, -prev.radius, point.x_coord, point.y_coord)
                    },
                    _ => {
                    format!(" L {} {}", point.x_coord, point.y_coord)                   }
                }};
            if prev.bevel > 0.0 {
                let bevel_line = format!(
                    "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"red\" stroke-width=\"4\" />",
                    prev.x_coord, prev.y_coord, point.x_coord, point.y_coord
                );
                bevel_lines.push(bevel_line);
            }
            path.push_str(&segment);
            (path, point.clone())
        },
    );

    format!(
        "<path d=\"{}\" fill=\"{}\" stroke=\"black\" stroke-width=\"0.5\" />{}",
        path_str.trim(),
        color,
        bevel_lines.join("")
    )
}

impl OuterBorder {
    /// Creates a new OuterBorder from a vector of BorderPoints
    /// # Arguments
    /// * `lines` - A vector of string slices, representing the contour of the border
    pub fn from_lines(lines: &[&str]) -> Self {
        Self {
            contour: read_contour(lines),
        }
    }
}

impl InnerBorder {
    /// Creates a new InnerBorder from a vector of BorderPoints
    /// # Arguments
    /// * `lines` - A vector of string slices, representing the contour of the border
    pub fn from_lines(lines: &[&str]) -> Self {
        Self {
            contour: read_contour(lines),
        }
    }
}
impl DstvElement for OuterBorder {
    /// Converts the outer border to an SVG path
    /// # Returns
    /// A string representing the SVG path of the outer border
    fn to_svg(&self) -> String {
        contour_to_svg(&self.contour, "grey")
    }

    fn from_str(_line: &str) -> Result<Self, &'static str> {
        todo!("Find out how to split traits and casts when when calling in a idiomatic way");
    }

    fn get_index(&self) -> usize {
        0
    }
}

impl DstvElement for InnerBorder {
    /// Converts the inner border to an SVG path
    /// # Returns
    /// A string representing the SVG path of the inner border
    fn to_svg(&self) -> String {
        contour_to_svg(&self.contour, "white")
    }

    fn from_str(_line: &str) -> Result<Self, &'static str> {
        todo!("Find out how to split traits and casts when when calling in a idiomatic way");
    }

    fn get_index(&self) -> usize {
        1
    }
}
