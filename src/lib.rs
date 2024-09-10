mod bend;
mod border;
mod cut;
mod dstv;
mod dstv_element;
mod element_type;
mod header;
mod hole;
mod numeration;
mod part_face;
mod slot;

use std::str::FromStr;

use prelude::ParseDstvError;

/// Re-export all the modules
pub mod prelude {
    pub use crate::bend::*;
    pub use crate::border::*;
    pub use crate::cut::*;
    pub use crate::dstv::*;
    pub use crate::dstv_element::*;
    pub use crate::element_type::*;
    pub use crate::header::*;
    pub use crate::hole::*;
    pub use crate::numeration::*;
    pub use crate::part_face::*;
    pub use crate::slot::*;
}

/// Validate if flange is either u v o or h
/// other values are not allowed
/// # arguments
/// * `flange` - flange code
/// # return
/// * `bool` - true if flange is valid
/// # example
/// ```
/// use dstv::validate_flange;
/// assert_eq!(validate_flange("u"), true);
/// assert_eq!(validate_flange("v"), true);
/// assert_eq!(validate_flange("o"), true);
/// assert_eq!(validate_flange("h"), true);
/// assert_eq!(validate_flange("x"), false);
/// ```
pub fn validate_flange(flange: &str) -> bool {
    part_face::PartFace::from_str(flange).is_ok()
}

/// Get f64 from string and strips it of any non numeric characters
/// # arguments
/// * `line` - line to parse
/// * `name` - name of the element
/// # return
/// * `f64` - parsed f64
/// # example
/// ```
/// use dstv::get_f64_from_str;
/// assert_eq!(get_f64_from_str(Some("1.0s"), "test"), Ok(1.0));
/// assert_eq!(get_f64_from_str(Some("1.0u"), "test"), Ok(1.0));
/// assert_eq!(get_f64_from_str(Some("1.0o"), "test"), Ok(1.0));
/// assert_eq!(get_f64_from_str(Some("1.0"), "test"), Ok(1.0));
/// assert_eq!(get_f64_from_str(None, "test"), Ok(0.0));
/// ```
pub fn get_f64_from_str(line: Option<&str>, name: &str) -> Result<f64, ParseDstvError> {
    match line {
        Some(x) => x
            .replace("s", "")
            .replace("w", "")
            .replace("l", "")
            .replace("u", "")
            .replace("o", "")
            .parse::<f64>()
            .map_err(|_| ParseDstvError::new(format!("`{name}` not a f64: got `{x}`"))),
        None => {
            println!("{} not found", name);
            Ok(0.0)
        }
    }
}
