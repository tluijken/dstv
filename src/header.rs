use std::{fmt::Debug, str::FromStr};

use crate::dstv_element::ParseDstvError;

#[derive(Debug, PartialEq, Eq, Hash)]
/// All available profiles in a DSTV file
pub enum CodeProfile {
    I,  // I Profiles
    L,  // L Profiles
    U,  // U and C Profiles
    B,  // Plate profiles
    RU, // Round tubes
    RO, // Round bars
    M,  // Rectangular tubes
    C,  // C Profiles
    T,  // T Profiles
    SO, // Z Profiles and all the other types of profile
}

impl CodeProfile {
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

impl FromStr for CodeProfile {
    type Err = ParseDstvError;
    /// Creates a new CodeProfile from a string slice
    /// # Arguments
    /// * `s` - A string slice that holds the CodeProfile
    /// # Returns
    /// * A CodeProfile
    /// # Error
    /// * If the string slice does not match any CodeProfile
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        Ok(match s {
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
            _ => return Err(ParseDstvError::new(format!("Invalid CodeProfile: {s}"))),
        })
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
    pub fn from_lines(lines: Vec<&str>) -> Result<Self, ParseDstvError> {
        if lines.len() < 24 {
            return Err(ParseDstvError::new("Invalid Header"));
        }
        let lengths: String = dstv_str(lines.get(8))?;
        let lengths = lengths.split_once(',');
        // the length and the saw length of the piece are stored on the same line,
        // separated by a comma
        let (length, saw_length) = match lengths {
            Some((one, two)) => (dstv_str(Some(one))?, dstv_str(Some(two)).ok()),
            _ => (dstv_str(lines.get(8))?, None),
        };

        Ok(Self {
            order_identification: dstv_str(lines.get(0))?,
            drawing_identification: dstv_str(lines.get(1))?,
            phase_identification: dstv_str(lines.get(2))?,
            piece_identification: dstv_str(lines.get(3))?,
            steel_quality: dstv_str(lines.get(4))?,
            quantity_of_pieces: dstv_str(lines.get(5))?,
            profile: dstv_str(lines.get(6))?,
            code_profile: {
                let s: String = dstv_str(lines.get(7))?;
                CodeProfile::from_str(&s)?
            },
            length,
            saw_length,
            profile_height: dstv_str(lines.get(9))?,
            flange_width: dstv_str(lines.get(10))?,
            flange_thickness: dstv_str(lines.get(11))?,
            web_thickness: dstv_str(lines.get(12))?,
            radius: dstv_str(lines.get(13))?,
            weight_by_meter: dstv_str(lines.get(14))?,
            painting_surface_by_meter: dstv_str(lines.get(15))?,
            web_start_cut: dstv_str(lines.get(16))?,
            web_end_cut: dstv_str(lines.get(17))?,
            flange_start_cut: dstv_str(lines.get(18))?,
            flange_end_cut: dstv_str(lines.get(19))?,
            text1_info_on_piece: dstv_str(lines.get(20))?,
            text2_info_on_piece: dstv_str(lines.get(21))?,
            text3_info_on_piece: dstv_str(lines.get(22))?,
            text4_info_on_piece: dstv_str(lines.get(23))?,
        })
    }
}

fn dstv_str<S, T>(inp: Option<S>) -> Result<T, ParseDstvError>
where
    S: AsRef<str>,
    T: FromStr,
    T::Err: Debug,
{
    inp.ok_or_else(|| ParseDstvError::new("Invalid Header"))?
        .as_ref()
        .trim()
        .parse()
        .map_err(|e| ParseDstvError::from_err("Could not parse from DSTV", e))
}
