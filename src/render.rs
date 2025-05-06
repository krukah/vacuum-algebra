/// idk what representation will look like
/// but this trait will represent how we will map
/// Expression -> PartOfBiggerPicture
pub trait Line {
    fn beg(&self) -> (usize, usize);
    fn end(&self) -> (usize, usize);
    fn scale(&self) -> f32;
}

/// i suppose this will be used to render paths,
/// given that these are the only degrees of freedom
pub struct Render {
    beg: (usize, usize),
    end: (usize, usize),
    scale: f32,
}

/// we treat each [Expression] bitstring as a tree path
/// such that it can be mapped into a
/// beginning and ending point on the canvas.
///
/// depending on how we shape the tree
/// (think: right angles, half-plane, dots, etc.)
/// we may implement coordinate calculation differently
/// but color is always just a function of the expression's expectation
impl<T> From<T> for Render
where
    T: Line,
{
    fn from(x: T) -> Self {
        Self {
            beg: x.beg(),
            end: x.end(),
            scale: x.scale(),
        }
    }
}
