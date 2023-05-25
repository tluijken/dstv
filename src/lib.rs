mod dstv;
mod dstv_element;
mod element_type;
mod header;
mod prelude {
    pub use crate::dstv::*;
    pub use crate::dstv_element::*;
    pub use crate::element_type::*;
    pub use crate::header::*;
}

use prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_header() {
        let dstv = Dstv::from_file("./data/E1.nc");
    }
}
