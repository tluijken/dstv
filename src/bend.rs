use crate::dstv_element::ParseDstvError;
use crate::get_f64_from_str;
use crate::prelude::DstvElement;

/// A bend is a circular arc.
/// It is defined by the angle of the arc, the radius of the arc, and the start and end points of the arc.
/// The start point is the origin of the arc.
/// The end point is the point where the arc ends.
/// The arc is drawn counter-clockwise from the origin to the end point.
/// The arc is drawn clockwise from the end point to the origin.
#[derive(Debug)]
pub struct Bend {
    /// The angle of the arc in degrees.
    pub angle: f64,
    /// The radius of the arc.
    pub radius: f64,
    /// The x-coordinate of the end point of the arc.
    pub finish_x: f64,
    /// The y-coordinate of the end point of the arc.
    pub finish_y: f64,
    /// The x-coordinate of the origin of the arc.
    pub origin_x: f64,
    /// The y-coordinate of the origin of the arc.
    pub origin_y: f64,
}

impl DstvElement for Bend {
    /// Create a new bend from a string slice.
    /// The string slice is expected to be a line from a DSTV file.
    /// # Arguments
    /// * `line` - A string slice containing a line from a DSTV file.
    /// # Returns
    /// A Result containing either a Bend or a &'static str.
    fn from_str(line: &str) -> Result<Self, ParseDstvError> {
        let mut iter = line.split_whitespace();
        let origin_x = get_f64_from_str(iter.next(), "origin_x")?;
        let origin_y = get_f64_from_str(iter.next(), "origin_y")?;
        let angle = get_f64_from_str(iter.next(), "angle")?;
        let radius = get_f64_from_str(iter.next(), "radius")?;
        let finish_x = get_f64_from_str(iter.next(), "finish_x")?;
        let finish_y = get_f64_from_str(iter.next(), "finish_y")?;
        Ok(Self {
            angle,
            radius,
            finish_x,
            finish_y,
            origin_x,
            origin_y,
        })
    }

    fn get_index(&self) -> usize {
        2
    }

    fn get_facing(&self) -> &crate::prelude::PartFace {
        &crate::prelude::PartFace::Top
    }

    /// Convert the bend to an SVG path.
    /// # Returns
    /// A string containing an SVG path.
    fn to_svg(&self) -> String {
        format!(
            "<path d=\"M{},{} A{},{},0,0,1,{},{}\" stroke=\"black\" fill=\"none\" />",
            self.origin_x, self.origin_y, self.radius, self.radius, self.finish_x, self.finish_y
        )
    }
}
