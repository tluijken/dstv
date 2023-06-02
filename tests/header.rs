#[cfg(test)]
mod tests {
    use dstv::prelude::*;
    #[test]
    fn read_header_p1() {
        let dstv = Dstv::from_file("./tests/data/P1.nc");
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

    #[test]
    fn read_header_rst37_2() {
        let dstv = Dstv::from_file("./tests/data/RST37-2.nc");
        assert_eq!(dstv.is_ok(), true);
        let dstv = dstv.unwrap();
        assert_eq!(dstv.header.order_identification, "1");
        assert_eq!(dstv.header.drawing_identification, "1");
        assert_eq!(dstv.header.phase_identification, "14");
        assert_eq!(dstv.header.piece_identification, "14");
        assert_eq!(dstv.header.steel_quality, "RST37-2");
        assert_eq!(dstv.header.quantity_of_pieces, 2);
        assert_eq!(dstv.header.profile, "ZS175*1.5");
        assert_eq!(dstv.header.code_profile, CodeProfile::SO);
        assert_eq!(dstv.header.length, 1133.00);
        assert_eq!(dstv.header.saw_length, None);
        assert_eq!(dstv.header.profile_height, 175.00);
        assert_eq!(dstv.header.flange_width, 81.00);
        assert_eq!(dstv.header.flange_thickness, 1.50);
        assert_eq!(dstv.header.web_thickness, 1.50);
        assert_eq!(dstv.header.radius, 4.00);
        assert_eq!(dstv.header.weight_by_meter, 4.416);
        assert_eq!(dstv.header.painting_surface_by_meter, 0.753);
        assert_eq!(dstv.header.web_start_cut, 0.000);
        assert_eq!(dstv.header.web_end_cut, 0.000);
        assert_eq!(dstv.header.flange_start_cut, 0.000);
        assert_eq!(dstv.header.flange_end_cut, 0.000);
        assert_eq!(dstv.header.text1_info_on_piece, "Pfette");
        assert_eq!(dstv.header.text2_info_on_piece, "");
        assert_eq!(dstv.header.text3_info_on_piece, "");
        assert_eq!(dstv.header.text4_info_on_piece, "");
    }

    #[test]
    fn read_header_pl01() {
        let dstv = Dstv::from_file("./tests/data/0008-PL0001.NC1");
        assert_eq!(dstv.is_ok(), true);
        let dstv = dstv.unwrap();
        assert_eq!(dstv.header.order_identification, "0008");
        assert_eq!(dstv.header.drawing_identification, "NA");
        assert_eq!(dstv.header.phase_identification, "NA");
        assert_eq!(dstv.header.piece_identification, "0008-PL0001");
        assert_eq!(dstv.header.steel_quality, "NA");
        assert_eq!(dstv.header.quantity_of_pieces, 3);
        assert_eq!(dstv.header.profile, "196.451x10 PL");
        assert_eq!(dstv.header.code_profile, CodeProfile::B);
        assert_eq!(dstv.header.length, 288.224);
        assert_eq!(dstv.header.saw_length, None);
        assert_eq!(dstv.header.profile_height, 10.0);
        assert_eq!(dstv.header.flange_width, 196.451);
        assert_eq!(dstv.header.flange_thickness, 0.0);
        assert_eq!(dstv.header.web_thickness, 10.0);
        assert_eq!(dstv.header.radius, 0.0);
        assert_eq!(dstv.header.weight_by_meter, 0.0);
        assert_eq!(dstv.header.painting_surface_by_meter, 0.0);
        assert_eq!(dstv.header.web_start_cut, 0.0);
        assert_eq!(dstv.header.web_end_cut, 0.0);
        assert_eq!(dstv.header.flange_start_cut, 0.0);
        assert_eq!(dstv.header.flange_end_cut, 0.0);
        assert_eq!(dstv.header.text1_info_on_piece, "");
        assert_eq!(dstv.header.text2_info_on_piece, "");
        assert_eq!(dstv.header.text3_info_on_piece, "");
    }

    #[test]
    fn read_header_se04() {
        let dstv = Dstv::from_file("./tests/data/0008-SE0004.nc1");
        assert_eq!(dstv.is_ok(), true);
        let dstv = dstv.unwrap();
        assert_eq!(dstv.header.order_identification, "0008");
        assert_eq!(dstv.header.drawing_identification, "NA");
        assert_eq!(dstv.header.phase_identification, "NA");
        assert_eq!(dstv.header.piece_identification, "0008-SE0004");
        assert_eq!(dstv.header.steel_quality, "NA");
        assert_eq!(dstv.header.quantity_of_pieces, 1);
        assert_eq!(dstv.header.profile, "UPE 200");
        assert_eq!(dstv.header.code_profile, CodeProfile::U);
        assert_eq!(dstv.header.length, 322.25);
        assert_eq!(dstv.header.saw_length, None);
        assert_eq!(dstv.header.profile_height, 200.0);
        assert_eq!(dstv.header.flange_width, 80.0);
        assert_eq!(dstv.header.flange_thickness, 11.0);
        assert_eq!(dstv.header.web_thickness, 6.0);
        assert_eq!(dstv.header.radius, 13.0);
        assert_eq!(dstv.header.weight_by_meter, 0.0);
        assert_eq!(dstv.header.painting_surface_by_meter, 0.0);
        assert_eq!(dstv.header.web_start_cut, 0.0);
        assert_eq!(dstv.header.web_end_cut, 0.0);
        assert_eq!(dstv.header.flange_start_cut, 0.0);
        assert_eq!(dstv.header.flange_end_cut, 0.0);
        assert_eq!(dstv.header.text1_info_on_piece, "");
        assert_eq!(dstv.header.text2_info_on_piece, "");
        assert_eq!(dstv.header.text3_info_on_piece, "");
    }

    #[test]
    fn read_header_se08() {
        let dstv = Dstv::from_file("./tests/data/0008-SE0008.nc1");
        assert_eq!(dstv.is_ok(), true);
        let dstv = dstv.unwrap();
        assert_eq!(dstv.header.order_identification, "0008");
        assert_eq!(dstv.header.drawing_identification, "NA");
        assert_eq!(dstv.header.phase_identification, "NA");
        assert_eq!(dstv.header.piece_identification, "0008-SE0008");
        assert_eq!(dstv.header.steel_quality, "NA");
        assert_eq!(dstv.header.quantity_of_pieces, 1);
        assert_eq!(dstv.header.profile, "IPE  300");
        assert_eq!(dstv.header.code_profile, CodeProfile::I);
        assert_eq!(dstv.header.length, 954.5);
        assert_eq!(dstv.header.saw_length, None);
        assert_eq!(dstv.header.profile_height, 300.0);
        assert_eq!(dstv.header.flange_width, 150.0);
        assert_eq!(dstv.header.flange_thickness, 10.7);
        assert_eq!(dstv.header.web_thickness, 7.1);
        assert_eq!(dstv.header.radius, 15.0);
        assert_eq!(dstv.header.weight_by_meter, 0.0);
        assert_eq!(dstv.header.painting_surface_by_meter, 0.0);
        assert_eq!(dstv.header.web_start_cut, 0.0);
        assert_eq!(dstv.header.web_end_cut, 0.0);
        assert_eq!(dstv.header.flange_start_cut, 0.0);
        assert_eq!(dstv.header.flange_end_cut, 0.0);
        assert_eq!(dstv.header.text1_info_on_piece, "");
        assert_eq!(dstv.header.text2_info_on_piece, "");
        assert_eq!(dstv.header.text3_info_on_piece, "");
    }

    #[test]
    fn read_header_se09() {
        let dstv = Dstv::from_file("./tests/data/0008-SE0009.nc1");
        assert_eq!(dstv.is_ok(), true);
        let dstv = dstv.unwrap();
        assert_eq!(dstv.header.order_identification, "0008");
        assert_eq!(dstv.header.drawing_identification, "NA");
        assert_eq!(dstv.header.phase_identification, "NA");
        assert_eq!(dstv.header.piece_identification, "0008-SE0009");
        assert_eq!(dstv.header.steel_quality, "NA");
        assert_eq!(dstv.header.quantity_of_pieces, 1);
        assert_eq!(dstv.header.profile, "100x100x6 SHS");
        assert_eq!(dstv.header.code_profile, CodeProfile::M);
        assert_eq!(dstv.header.length, 1000.0);
        assert_eq!(dstv.header.saw_length, None);
        assert_eq!(dstv.header.profile_height, 100.0);
        assert_eq!(dstv.header.flange_width, 100.0);
        assert_eq!(dstv.header.flange_thickness, 6.0);
        assert_eq!(dstv.header.web_thickness, 6.0);
        assert_eq!(dstv.header.radius, 7.5);
        assert_eq!(dstv.header.weight_by_meter, 0.0);
        assert_eq!(dstv.header.painting_surface_by_meter, 0.0);
        assert_eq!(dstv.header.web_start_cut, 0.0);
        assert_eq!(dstv.header.web_end_cut, 0.0);
        assert_eq!(dstv.header.flange_start_cut, 0.0);
        assert_eq!(dstv.header.flange_end_cut, 0.0);
        assert_eq!(dstv.header.text1_info_on_piece, "");
        assert_eq!(dstv.header.text2_info_on_piece, "");
        assert_eq!(dstv.header.text3_info_on_piece, "");
    }
}
