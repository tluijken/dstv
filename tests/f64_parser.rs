#[cfg(test)]
mod tests {
    use dstv::get_f64_from_str;

    #[test]
    fn test_f64_parsing() {
        assert_eq!(get_f64_from_str(Some("1.0s"), "test"), 1.0);
        assert_eq!(get_f64_from_str(Some("1.0u"), "test"), 1.0);
        assert_eq!(get_f64_from_str(Some("1.0o"), "test"), 1.0);
        assert_eq!(get_f64_from_str(Some("1.0"), "test"), 1.0);
        assert_eq!(get_f64_from_str(None, "test"), 0.0);
    }
}
