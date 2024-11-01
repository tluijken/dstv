use crate::dstv_element::ParseDstvError;
use crate::get_f64_from_str;
use crate::prelude::DstvElement;

/// Represents a cut in the DSTV file
#[derive(Debug)]
pub struct Cut {
    /// Normal vector x component
    pub nor_vec_x: f64,
    /// Normal vector y component
    pub nor_vec_y: f64,
    /// Normal vector z component
    pub nor_vec_z: f64,
    /// Start point x coordinate
    pub sp_point_x: f64,
    /// Start point y coordinate
    pub sp_point_y: f64,
    /// Start point z coordinate
    pub sp_point_z: f64,
}

impl DstvElement for Cut {
    /// Create a new cut from a string slice.
    /// The string slice is expected to be a line from a DSTV file.
    /// # Arguments
    /// * `line` - A string slice containing a line from a DSTV file.
    /// # Returns
    /// A Result containing either a Cut or a &'static str.
    fn from_str(line: &str) -> Result<Self, ParseDstvError> {
        let mut iter = line.split_whitespace();
        if iter.clone().count() < 6 {
            return Err(ParseDstvError::new(
                "Illegal data vector format (SC): too short",
            ));
        }
        let sp_point_x = get_f64_from_str(iter.next(), "sp_point_x")?;
        let sp_point_y = get_f64_from_str(iter.next(), "sp_point_y")?;
        let sp_point_z = get_f64_from_str(iter.next(), "sp_point_z")?;
        let nor_vec_x = get_f64_from_str(iter.next(), "nor_vec_x")?;
        let nor_vec_y = get_f64_from_str(iter.next(), "nor_vec_y")?;
        let nor_vec_z = get_f64_from_str(iter.next(), "nor_vec_z")?;
        Ok(Self {
            nor_vec_x,
            nor_vec_y,
            nor_vec_z,
            sp_point_x,
            sp_point_y,
            sp_point_z,
        })
    }

    /// Convert the cut to an SVG path.
    /// # Returns
    /// A string containing an SVG path.
    fn to_svg(&self) -> String {
        format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"black\" />",
            self.sp_point_x,
            self.sp_point_y,
            self.sp_point_x + self.nor_vec_x,
            self.sp_point_y + self.nor_vec_y
        )
    }

    fn get_index(&self) -> usize {
        2
    }

    fn get_facing(&self) -> &crate::prelude::PartFace {
        &crate::prelude::PartFace::Top
    }
}
