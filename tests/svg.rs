#[cfg(test)]
mod tests {
    use dstv::prelude::*;
    #[test]
    fn read_svg_p82() {
        let dstv = Dstv::from_file("./tests/data/RST37-2.nc").unwrap();
        let svg = dstv.to_svg();
        std::fs::write("tests/example/RST37-2.svg", svg).unwrap();
    }
}
