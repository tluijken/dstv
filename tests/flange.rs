#[cfg(test)]
mod tests {
    use dstv::validate_flange;

    #[test]
    fn test_validate_flange() {
        assert_eq!(validate_flange("u"), true);
        assert_eq!(validate_flange("v"), true);
        assert_eq!(validate_flange("o"), true);
        assert_eq!(validate_flange("h"), true);
        assert_eq!(validate_flange("x"), false);
    }
}
