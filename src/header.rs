#[derive(Debug, PartialEq, Eq, Hash)]
/// All available profiles in a DSTV file
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
    /// Creates a new CodeProfile from a string slice
    /// # Arguments
    /// * `s` - A string slice that holds the CodeProfile
    /// # Returns
    /// * A CodeProfile
    /// # Panics
    /// * If the string slice does not match any CodeProfile
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

    /// Converts a CodeProfile to a string slice
    /// # Arguments
    /// * `self` - A CodeProfile
    /// # Returns
    /// * A string slice for a more human readable CodeProfile
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

/// The header of a DSTV file
#[derive(Debug)]
pub struct Header {
    /// The order identification of the order the pieces belong to
    pub order_identification: String,
    /// The drawing identification of the drawing the pieces belong to
    pub drawing_identification: String,
    /// The phase identification of the phase the pieces belong to
    pub phase_identification: String,
    /// The piece identification of the piece
    pub piece_identification: String,
    /// The steel quality of the piece
    pub steel_quality: String,
    /// The number of pieces for the order, and phase
    pub quantity_of_pieces: i32,
    /// The profile of the piece
    pub profile: String,
    /// The profile type of the piece
    pub code_profile: CodeProfile,
    /// The length of the piece
    pub length: f64,
    /// The saw length of the piece
    pub saw_length: Option<f64>,
    /// The profile height of the piece
    pub profile_height: f64,
    /// The flange width of the piece
    pub flange_width: f64,
    /// The flange thickness of the piece
    pub flange_thickness: f64,
    /// The web thickness of the piece
    pub web_thickness: f64,
    /// The radius of the piece
    pub radius: f64,
    /// The weight by meter of the piece
    pub weight_by_meter: f64,
    /// The painting surface by meter of the piece
    pub painting_surface_by_meter: f64,
    /// The web start cut of the piece
    pub web_start_cut: f64,
    /// The web end cut of the piece
    pub web_end_cut: f64,
    /// The flange start cut of the piece
    pub flange_start_cut: f64,
    /// The flange end cut of the piece
    pub flange_end_cut: f64,
    /// The text1 info on piece of the piece
    pub text1_info_on_piece: String,
    /// The text2 info on piece of the piece
    pub text2_info_on_piece: String,
    /// The text3 info on piece of the piece
    pub text3_info_on_piece: String,
    /// The text4 info on piece of the piece
    pub text4_info_on_piece: String,
}

impl Header {
    /// Creates a new Header from a vector of string slices
    /// # Arguments
    /// * `lines` - A vector of string slices that holds the header
    /// # Returns
    /// A `Result` containing either a `Header` or an error message
    pub fn from_lines(lines: Vec<&str>) -> Result<Self, &'static str> {
        if lines.len() < 24 {
            return Err("Invalid Header");
        }
        let lengths = lines[8].trim().split(',').collect::<Vec<&str>>();
        // the length and the saw length of the piece are stored on the same line,
        // separated by a comma
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
