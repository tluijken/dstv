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

/// calculates the bend between two border points if the previous border point has a radius.
/// # Arguments
/// * `point` - The current border point
/// * `prev` - The previous border point
/// # Returns
/// A tuple of four f64 values representing the bend
fn get_bend(point: &BorderPoint, prev: &BorderPoint) -> (f64, f64, f64, f64) {
    match (prev.y_coord > point.y_coord, point.x_coord > prev.x_coord) {
        (true, true) => (prev.x_coord, point.y_coord, point.x_coord, point.y_coord), // left-top corner
        (false, true) => (point.x_coord, prev.y_coord, point.x_coord, point.y_coord), // top-right corner
        (false, false) => (prev.x_coord, point.y_coord, point.x_coord, point.y_coord), // right-bottom corner
        (true, false) => (point.x_coord, prev.y_coord, point.x_coord, point.y_coord), // bottom-left corner
    }
}

/// Converts a contour to an SVG path
/// # Arguments
/// * `contour` - A vector of BorderPoints, representing the contour of a border
/// * `color` - A string representing the color of the border
/// # Returns
/// A string representing the SVG path of the border
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
}
