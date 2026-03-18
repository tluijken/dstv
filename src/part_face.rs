use crate::prelude::ParseDstvError;
use std::str::FromStr;

/// Identifies which physical face of the steel part an element belongs to.
///
/// DSTV files use single-character codes to denote each face. The four faces
/// correspond to the sides of the profile cross-section:
///
/// ```text
///        ┌──── Top (o) ────┐
///        │                 │
/// Front  │                 │  Behind
///  (v)   │                 │   (h)
///        │                 │
///        └─── Bottom (u) ──┘
/// ```
///
/// For profiles such as I-beams, Top and Bottom are the flanges and Front/Behind
/// are the web faces. For flat plates only the Front face is typically used.
///
/// In the SVG output the faces are stacked vertically in the order:
/// Bottom → Front → Top → Behind (see [`Dstv::to_svg`]).
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PartFace {
    /// The front face — DSTV code `v`. For I/H profiles this is the front web face.
    Front,
    /// The top face — DSTV code `o`. For I/H profiles this is the top flange.
    /// Elements on this face have their Y-axis flipped in the SVG output so that
    /// Y=0 (the inner/web edge) aligns correctly with the Front face below it.
    Top,
    /// The bottom face — DSTV code `u`. For I/H profiles this is the bottom flange.
    Bottom,
    /// The behind face — DSTV code `h`. For I/H profiles this is the rear web face.
    Behind,
}

impl FromStr for PartFace {
    type Err = ParseDstvError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        match s {
            "v" => Ok(PartFace::Front),
            "o" => Ok(PartFace::Top),
            "u" => Ok(PartFace::Bottom),
            "h" => Ok(PartFace::Behind),
            _ => Err(ParseDstvError::new(format!("Invalid Face: {s}"))),
        }
    }
}

impl Default for PartFace {
    fn default() -> Self {
        Self::Front
    }
}
