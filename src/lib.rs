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
        let dstv = Dstv::from_file("./data/P1.nc");
        assert_eq!(dstv.is_ok(), true);
        let dstv = dstv.unwrap();
        assert_eq!(dstv.header.order_identification, "PROJECT-1");
        assert_eq!(dstv.header.drawing_identification, "0");
        assert_eq!(dstv.header.phase_identification, "1");
        assert_eq!(dstv.header.piece_identification, "B_1");
        assert_eq!(dstv.header.steel_quality, "A992");
        assert_eq!(dstv.header.quantity_of_pieces, 1);
        assert_eq!(dstv.header.profile, "W21X44");
        assert_eq!(dstv.header.code_profile, CodeProfile::I);
        assert_eq!(dstv.header.length, 6236.88);
        assert_eq!(dstv.header.saw_length, None);
        assert_eq!(dstv.header.profile_height, 525.78);
        assert_eq!(dstv.header.flange_width, 165.10);
        assert_eq!(dstv.header.flange_thickness, 11.43);
        assert_eq!(dstv.header.web_thickness, 8.89);
        assert_eq!(dstv.header.radius, 17.15);
        assert_eq!(dstv.header.weight_by_meter, 65.479);
        assert_eq!(dstv.header.painting_surface_by_meter, 0.000);
        assert_eq!(dstv.header.web_start_cut, 0.000);
        assert_eq!(dstv.header.web_end_cut, 15.000);
        assert_eq!(dstv.header.flange_start_cut, 0.000);
        assert_eq!(dstv.header.flange_end_cut, 0.000);
        assert_eq!(dstv.header.text1_info_on_piece, "");
        assert_eq!(dstv.header.text2_info_on_piece, "");
        assert_eq!(dstv.header.text3_info_on_piece, "");
    }
}
