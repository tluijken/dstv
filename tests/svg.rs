#[cfg(test)]
mod tests {
    use std::fs;

    use dstv::prelude::*;

    #[test]
    fn read_svg_p465() {
        let mut dstv = Dstv::from_file("./tests/data/P465.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/P465.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

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

    #[test]
    fn read_svg_p1565() {
        let mut dstv = Dstv::from_file("./tests/data/P1565.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/p1565.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_p1719() {
        let mut dstv = Dstv::from_file("./tests/data/P1719.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/p1719.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_p1728() {
        let mut dstv = Dstv::from_file("./tests/data/P1728.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/p1728.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }
    #[test]
    fn read_svg_p1730() {
        let mut dstv = Dstv::from_file("./tests/data/P1730.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/p1730.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_p2663() {
        let mut dstv = Dstv::from_file("./tests/data/P2663.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/P2663.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_p2683() {
        let mut dstv = Dstv::from_file("./tests/data/P2683.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/P2683.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }

    #[test]
    fn read_svg_p2684() {
        let mut dstv = Dstv::from_file("./tests/data/P2684.nc").unwrap();
        let svg = dstv.to_svg();
        assert_eq!(
            svg,
            fs::read_to_string("./tests/output_svg/P2684.svg")
                .unwrap()
                // trim the last newline
                .replace("\n", "")
        );
    }
}
