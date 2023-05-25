use crate::prelude::ElementType;

const Comment: char = '*';
const Space: char = ' ';

pub struct Header {
    pub project_identification: String,
    pub order_identification: String,
    pub drawing_identification: String,
    pub phase_identification: String,
    pub piece_identification: String,
    pub steel_quality: String,
    pub quantity_of_pieces: i32,
    pub profile: String,
    pub code_profile: String,
    pub length: f64,
    pub saw_length: f64,
    pub profile_height: f64,
    pub flange_width: f64,
    pub flange_thickness: f64,
    pub web_thickness: f64,
    pub radius: f64,
    pub weight_by_meter: f64,
    pub painting_surface_by_meter: f64,
    pub web_start_cut: f64,
    pub web_end_cut: f64,
    pub flange_start_cut: f64,
    pub flange_end_cut: f64,
    pub text1_info_on_piece: String,
    pub text2_info_on_piece: String,
    pub text3_info_on_piece: String,
    pub text4_info_on_piece: String,
}

impl Header {
    // pub fn from_lines(lines: Vec<&str>) -> Self {
    //     Self
    // }
}
