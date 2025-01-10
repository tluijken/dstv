#[cfg(test)]
mod tests {
    use dstv::get_f64_from_str;

    #[test]
    fn test_f64_parsing() {
        assert_eq!(get_f64_from_str(Some("1.0s"), "test"), Ok(1.0));
        assert_eq!(get_f64_from_str(Some("1.0u"), "test"), Ok(1.0));
        assert_eq!(get_f64_from_str(Some("1.0o"), "test"), Ok(1.0));
        assert_eq!(get_f64_from_str(Some("1.0"), "test"), Ok(1.0));
        assert_eq!(get_f64_from_str(None, "test"), Ok(0.0));
        assert_eq!(get_f64_from_str(Some("klj"), "test").is_err(), true);
    }
}
