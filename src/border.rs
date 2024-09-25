use crate::{dstv_element::{DstvElement, ParseDstvError}, get_f64_from_str, prelude::PartFace};
use std::str::FromStr;

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
    pub fl_code: PartFace,
    /// The x coordinate of the border point
    pub x_coord: f64,
    /// The y coordinate of the border point
    pub y_coord: f64,
    /// The radius of the border point
    pub radius: f64,
    /// The bevel of the border point between this and the next point.
    pub bevel: f64,
}

/// Reads the contour of a border from a DSTV file.
/// The contour is represented by a vector of BorderPoints
/// # Arguments
/// * `lines` - A vector of strings, representing the lines of the DSTV file
/// # Returns
/// A vector of BorderPoints, representing the contour of the border
/// # Errors
/// * If the flange code of a border point is invalid
/// * If the x coordinate of a border point is invalid
/// * If the y coordinate of a border point is invalid
/// * If the radius of a border point is invalid
fn read_contour(lines: &[&str]) -> Result<Vec<BorderPoint>,ParseDstvError> {
    lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut iter = line.split_whitespace().peekable();
            let first = iter.peek();
            let fl_code = match PartFace::from_str(first.unwrap_or(&"").trim()) {
                Ok(fl_code) => {
                    iter.next(); // iterate to the next split
                    fl_code
                }
                Err(_) => {
                    PartFace::Front
                }
            };

            let x_coord = get_f64_from_str(iter.next(), "x_coord")?;
            let y_coord = get_f64_from_str(iter.next(), "y_coord")?;
            let radius = get_f64_from_str(iter.next(), "radius")?;
            let bevel = match iter.next() {
                Some(val) =>     get_f64_from_str(Some(val), "bevel")?,
                _ => 0.0
            };
            Ok(BorderPoint {
                fl_code,
                x_coord,
                y_coord,
                radius,
                bevel,
            })
        })
        .collect()
}

/// Converts a contour to an SVG path
/// # Arguments
/// * `contour` - A vector of BorderPoints, representing the contour of a border
/// * `color` - A string representing the color of the border
/// # Returns
/// A string representing the SVG path of the border
fn contour_to_svg(contour: &[BorderPoint], color: &str, stroke_width: f64) -> String {
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
        "<path d=\"{}\" fill=\"{}\" stroke=\"black\" stroke-width=\"{}\" />{}",
        path_str.trim(),
        color,
        stroke_width,
        bevel_lines.join("")
    )
}

impl OuterBorder {
    /// Creates a new OuterBorder from a vector of BorderPoints
    /// # Arguments
    /// * `lines` - A vector of string slices, representing the contour of the border
    pub fn from_lines(lines: &[&str]) -> Result<Self,ParseDstvError> {
        Ok(Self {
            contour: read_contour(lines)?,
        })
    }
}

impl InnerBorder {
    /// Creates a new InnerBorder from a vector of BorderPoints
    /// # Arguments
    /// * `lines` - A vector of string slices, representing the contour of the border
    pub fn from_lines(lines: &[&str]) -> Result<Self,ParseDstvError> {
        Ok(Self {
            contour: read_contour(lines)?,
        })
    }
}
impl DstvElement for OuterBorder {
    /// Converts the outer border to an SVG path
    /// # Returns
    /// A string representing the SVG path of the outer border
    fn to_svg(&self) -> String {
        contour_to_svg(&self.contour, "grey", 0.5)
    }

    fn from_str(_line: &str) -> Result<Self, ParseDstvError> {
        todo!("Find out how to split traits and casts when when calling in a idiomatic way");
    }

    fn get_index(&self) -> usize {
        0
    }

    fn get_facing(&self) -> &PartFace {
        &self.contour[0].fl_code
    }
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

impl DstvElement for InnerBorder {
    /// Converts the inner border to an SVG path
    /// # Returns
    /// A string representing the SVG path of the inner border
    fn to_svg(&self) -> String {
        contour_to_svg(&self.contour, "white", 0.5)
    }

    fn from_str(_line: &str) -> Result<Self, ParseDstvError> {
        todo!("Find out how to split traits and casts when when calling in a idiomatic way");
    }

    fn get_index(&self) -> usize {
        1
    }

    fn get_facing(&self) -> &PartFace {
        &self.contour[0].fl_code
    }
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}
