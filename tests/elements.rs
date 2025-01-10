#[cfg(test)]
mod tests {
    use dstv::prelude::*;
    #[test]
    fn check_element_by_type() {
        let mut outer_border = 0;
        let mut inner_border = 0;
        let mut cut = 0;
        let mut bend = 0;
        let mut slot = 0;
        let mut hole = 0;
        let mut numeration = 0;

        let dstv = Dstv::from_file("./tests/data/P1565.nc").unwrap();
        for element in dstv.elements {
            match element {
                DstvElementType::OuterBorder(e) => 
                {
                    assert!(e.contour.len() > 0);
                    assert_eq!(e.get_index(), 0);
                    outer_border += 1;
                },
                DstvElementType::InnerBorder(e) => 
                {
                    assert!(e.contour.len() > 0);
                    assert_eq!(e.get_index(), 0);
                    inner_border += 1;
                },
                DstvElementType::Cut(e) => 
                {
                    assert!(e.nor_vec_x > 0.0);
                    assert_eq!(e.get_index(), 0);
                    cut += 1;
                },
                DstvElementType::Bend(e) => 
                {
                    assert!(e.angle > 0.0);
                    assert_eq!(e.get_index(), 0);
                    bend += 1;
                },
                DstvElementType::Slot(e) => 
                {
                    assert!(e.angle > 0.0);
                    assert_eq!(e.get_index(), 0);
                    slot += 1;
                },
                DstvElementType::Hole(e) => 
                {
                    assert!(e.diameter > 0.0);
                    assert_eq!(e.get_index(), 2);
                    hole += 1;
                },
                DstvElementType::Numeration(e) => 
                {
                    assert_ne!(e.text, "");
                    assert_eq!(e.get_index(), 2);
                    numeration += 1;
                },
            }
        }
        assert_eq!(outer_border, 1);
        assert_eq!(inner_border, 0);
        assert_eq!(cut, 0);
        assert_eq!(bend, 0);
        assert_eq!(slot, 0);
        assert_eq!(hole, 5);
        assert_eq!(numeration, 1);

    }
}
