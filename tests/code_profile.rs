#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use dstv::prelude::*;
    #[test]
    fn check_code_profile() {
        assert_eq!(CodeProfile::I.to_str(), "Profile I");
        assert_eq!(CodeProfile::L.to_str(), "Profile L");
        assert_eq!(CodeProfile::U.to_str(), "Profile U");
        assert_eq!(CodeProfile::B.to_str(), "Sheets, Plate, teared sheets, etc.");
        assert_eq!(CodeProfile::RU.to_str(), "Round");
        assert_eq!(CodeProfile::RO.to_str(), "Rounded Tube");
        assert_eq!(CodeProfile::M.to_str(), "Rectangular Tube");
        assert_eq!(CodeProfile::C.to_str(), "Profile C");
        assert_eq!(CodeProfile::T.to_str(), "Profile T");
        assert_eq!(CodeProfile::SO.to_str(), "Special Profile");
    }

    #[test]
    fn check_code_profile_from_str() {
        assert_eq!(CodeProfile::from_str("I").unwrap(), CodeProfile::I);
        assert_eq!(CodeProfile::from_str("L").unwrap(), CodeProfile::L);
        assert_eq!(CodeProfile::from_str("U").unwrap(), CodeProfile::U);
        assert_eq!(CodeProfile::from_str("B").unwrap(), CodeProfile::B);
        assert_eq!(CodeProfile::from_str("RU").unwrap(), CodeProfile::RU);
        assert_eq!(CodeProfile::from_str("RO").unwrap(), CodeProfile::RO);
        assert_eq!(CodeProfile::from_str("M").unwrap(), CodeProfile::M);
        assert_eq!(CodeProfile::from_str("C").unwrap(), CodeProfile::C);
        assert_eq!(CodeProfile::from_str("T").unwrap(), CodeProfile::T);
        assert_eq!(CodeProfile::from_str("SO").unwrap(), CodeProfile::SO);
        assert_eq!(CodeProfile::from_str("Invalid").is_err(), true);
    }
}
