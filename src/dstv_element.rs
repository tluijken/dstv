pub trait DstvElement {
    fn from_str(line: &str) -> Result<Self, &'static str>
    where
        Self: Sized;

    fn to_svg(&self) -> String;
}
