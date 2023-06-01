#[cfg(test)]
mod tests {
    use dstv::prelude::*;
    #[test]
    fn read_svg_p82() {
        let dstv = Dstv::from_file("./tests/data/P82.nc").unwrap();
        let svg = dstv.to_svg();
        std::fs::write("tests/example/p82-output.svg", svg).unwrap();
    }
}
