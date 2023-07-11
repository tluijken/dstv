pub trait DstvElement {
    /// Creates a new DSTV element from a string slice.
    /// # Arguments
    /// * `line` - A string slice that holds the line of the DSTV file
    /// # Returns
    /// * A DSTV element
    fn from_str(line: &str) -> Result<Self, &'static str>
    where
        Self: Sized;

    /// Convert the element to an SVG representation.
    /// # Returns
    /// A string containing an SVG representation of the element.
    fn to_svg(&self) -> String;

    /// Returns the index of the element in the DSTV file.
    /// This is used to determine the order in which the elements are drawn.
    /// The lower the index, the earlier the element is drawn.
    fn get_index(&self) -> usize {
        0
    }
}
