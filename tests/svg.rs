#[cfg(test)]
mod tests {
    use std::fs;

    use dstv::prelude::*;
    #[test]
    fn read_svg_rst37_2() {
        let mut dstv = Dstv::from_file("./tests/data/RST37-2.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/RST37-2.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_p1() {
        let mut dstv = Dstv::from_file("./tests/data/RST37-2.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/RST37-2.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_pl01() {
        let mut dstv = Dstv::from_file("./tests/data/0008-PL0001.NC1").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/0008-PL0001.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_se04() {
        let mut dstv = Dstv::from_file("./tests/data/0008-SE0004.nc1").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/0008-SE0004.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_se08() {
        let mut dstv = Dstv::from_file("./tests/data/0008-SE0008.nc1").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/0008-SE0008.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_se09() {
        let mut dstv = Dstv::from_file("./tests/data/0008-SE0009.nc1").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/0008-SE0009.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_product2() {
        let mut dstv = Dstv::from_file("./tests/data/product2.NC1").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/product-2.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_product3() {
        let mut dstv = Dstv::from_file("./tests/data/product3.NC1").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/product-3.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }
}
