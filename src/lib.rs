mod bend;
mod border;
mod cut;
mod dstv;
mod dstv_element;
mod element_type;
mod header;
mod hole;
mod numeration;
mod slot;

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
    pub use crate::slot::*;
}

pub fn validate_flange(flange: &str) -> bool {
    // validate is flange is either u v o or h
    match flange {
        "u" | "v" | "o" | "h" => true,
        _ => false,
    }
}

pub fn get_f64_from_str(line: Option<&str>, name: &str) -> f64 {
    match line {
        Some(x) => x
            .replace("s", "")
            .replace("u", "")
            .replace("o", "")
            .parse::<f64>()
            .expect(&format!("{} not a f64", name)),
        None => {
            println!("{} not found", name);
            0.0
        }
    }
}
