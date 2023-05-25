pub struct Contour {
    pub contour_type: ElementType,
    pub contour: Vec<Point>,
}

pub struct ContourPoint {
    pub fl_code: &str,
    pub x_coord: f64,
    pub y_coord: f64,
    pub radius: f64,
}
