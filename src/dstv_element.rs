pub trait DstvElement {
    fn from_lines(line: &str) -> Result<Self, &'static str>
    where
        Self: Sized;

    fn to_svg(&self) -> String;

    fn is_contour(&self) -> bool;
}
