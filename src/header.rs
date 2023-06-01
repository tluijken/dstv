#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CodeProfile {
    I,
    L,
    U,
    B,
    RU,
    RO,
    M,
    C,
    T,
    SO,
}

impl CodeProfile {
    fn from_str(s: &str) -> CodeProfile {
        match s {
            "I" => CodeProfile::I,
            "L" => CodeProfile::L,
            "U" => CodeProfile::U,
            "B" => CodeProfile::B,
            "RU" => CodeProfile::RU,
            "RO" => CodeProfile::RO,
            "M" => CodeProfile::M,
            "C" => CodeProfile::C,
            "T" => CodeProfile::T,
            "SO" => CodeProfile::SO,
            _ => panic!("Invalid CodeProfile"),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            CodeProfile::I => "Profile I",
            CodeProfile::L => "Profile L",
            CodeProfile::U => "Profile U",
            CodeProfile::B => "Sheets, Plate, teared sheets, etc.",
            CodeProfile::RU => "Round",
            CodeProfile::RO => "Rounded Tube",
            CodeProfile::M => "Rectangular Tube",
            CodeProfile::C => "Profile C",
            CodeProfile::T => "Profile T",
            CodeProfile::SO => "Special Profile",
        }
    }
}

#[derive(Debug)]
pub struct Header {
    pub order_identification: String,
    pub drawing_identification: String,
    pub phase_identification: String,
    pub piece_identification: String,
    pub steel_quality: String,
    pub quantity_of_pieces: i32,
    pub profile: String,
    pub code_profile: CodeProfile,
    pub length: f64,
    pub saw_length: Option<f64>,
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
    pub fn from_lines(lines: Vec<&str>) -> Result<Self, &'static str> {
        if lines.len() < 24 {
            return Err("Invalid Header");
        }
        let lengths = lines[8].trim().split(',').collect::<Vec<&str>>();
        let (length, saw_length) = if lengths.len() == 2 {
            (
                lengths[0].trim().parse::<f64>().unwrap(),
                Some(lengths[1].trim().parse::<f64>().unwrap()),
            )
        } else {
            (lengths[0].trim().parse::<f64>().unwrap(), None)
        };
        Ok(Self {
            order_identification: lines[0].trim().to_string(),
            drawing_identification: lines[1].trim().to_string(),
            phase_identification: lines[2].trim().to_string(),
            piece_identification: lines[3].trim().to_string(),
            steel_quality: lines[4].trim().to_string(),
            quantity_of_pieces: lines[5].trim().parse::<i32>().unwrap(),
            profile: lines[6].trim().to_string(),
            code_profile: CodeProfile::from_str(lines[7].trim()),
            length,
            saw_length,
            profile_height: lines[9].trim().parse::<f64>().unwrap(),
            flange_width: lines[10].trim().parse::<f64>().unwrap(),
            flange_thickness: lines[11].trim().parse::<f64>().unwrap(),
            web_thickness: lines[12].trim().parse::<f64>().unwrap(),
            radius: lines[13].trim().parse::<f64>().unwrap(),
            weight_by_meter: lines[14].trim().parse::<f64>().unwrap(),
            painting_surface_by_meter: lines[15].trim().parse::<f64>().unwrap(),
            web_start_cut: lines[16].trim().parse::<f64>().unwrap(),
            web_end_cut: lines[17].trim().parse::<f64>().unwrap(),
            flange_start_cut: lines[18].trim().parse::<f64>().unwrap(),
            flange_end_cut: lines[19].trim().parse::<f64>().unwrap(),
            text1_info_on_piece: lines[20].trim().to_string(),
            text2_info_on_piece: lines[21].trim().to_string(),
            text3_info_on_piece: lines[22].trim().to_string(),
            text4_info_on_piece: lines[23].trim().to_string(),
        })
    }
}
