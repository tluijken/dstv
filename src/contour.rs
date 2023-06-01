use crate::prelude::ElementType;

pub struct Contour {
    pub contour_type: ElementType,
    pub contour: Vec<ContourPoint>,
}

pub struct ContourPoint {
    pub fl_code: String,
    pub x_coord: f64,
    pub y_coord: f64,
    pub radius: f64,
}
