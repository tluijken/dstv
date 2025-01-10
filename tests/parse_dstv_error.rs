#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use dstv::prelude::*;

    #[test]
    fn read_invalid_hole() {
        let hole = Hole::from_str("");
        assert_eq!(hole.is_err(), true);
        assert_eq!(hole.err().unwrap().to_string(), "Could not parse DSTV element: \"No Hole Found\"");
    }

    #[test]
    fn read_invalid_cut() {
        let cut = Cut::from_str("");
        assert_eq!(cut.is_err(), true);
        assert_eq!(cut.err().unwrap().to_string(), "Could not parse DSTV element: \"Illegal data vector format (SC): too short\"");
    }

    #[test]
    fn read_invalid_header() {
        let dstv = Dstv::from_str("");
        assert_eq!(dstv.is_err(), true);
        assert_eq!(dstv.err().unwrap().to_string(), "Could not parse DSTV element: \"`Invalid Header`:\\t`ParseDstvError {\\n    message: \\\"Invalid Header\\\",\\n}`\"");
    }

    #[test]
    fn read_invalid_numeration() {
        let numeration = Numeration::from_str("");
        assert_eq!(numeration.is_err(), true);
        assert_eq!(numeration.err().unwrap().to_string(), "Could not parse DSTV element: \"No Numeration Found\"");
    }

    #[test]
    fn read_invalid_slot() {
        let slot = Slot::from_str("");
        assert_eq!(slot.is_err(), true);
        assert_eq!(slot.err().unwrap().to_string(), "Could not parse DSTV element: \"No Slot Found\"");
    }

    #[test]
    fn read_part_face() {
        let front = PartFace::from_str("v");
        assert_eq!(front.is_ok(), true);
        assert_eq!(front.unwrap(), PartFace::Front);

        let top = PartFace::from_str("o");
        assert_eq!(top.is_ok(), true);
        assert_eq!(top.unwrap(), PartFace::Top);

        let bottom = PartFace::from_str("u");
        assert_eq!(bottom.is_ok(), true);
        assert_eq!(bottom.unwrap(), PartFace::Bottom);

        let back = PartFace::from_str("h");
        assert_eq!(back.is_ok(), true);
        assert_eq!(back.unwrap(), PartFace::Behind);

        let left = PartFace::from_str("l");
        assert_eq!(left.is_err(), true);
        assert_eq!(left.err().unwrap().to_string(), "Could not parse DSTV element: \"Invalid Face: l\"");
    }

}
