use svg::node::element::Line;
use svg::Document;

const fn w() -> usize {
    1024
}
const fn h() -> usize {
    1024
}

/// idk what representation will look like
/// but this trait will represent how we will map
/// Expression -> PartOfBiggerPicture
pub trait Segment {
    fn beg(&self) -> (f32, f32);
    fn end(&self) -> (f32, f32);
    fn scale(&self) -> f32;
    fn stroke(&self) -> f32;
}

pub fn draw<T, I>(lines: I) -> std::io::Result<()>
where
    T: Segment,
    I: IntoIterator<Item = T>,
{
    let ref filename = "tree.svg";
    let ref image = lines
        .into_iter()
        .map(render)
        .fold(canvas(), |doc, line| doc.add(line));
    svg::save(filename, image).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}

fn render<T>(line: T) -> Line
where
    T: Segment,
{
    Line::new()
        .set("x1", line.beg().0)
        .set("y1", line.beg().1)
        .set("x2", line.end().0)
        .set("y2", line.end().1)
        .set("stroke", line.scale())
        .set("stroke-width", line.stroke())
}

fn canvas() -> Document {
    Document::new()
        .set("viewBox", (0, 0, w(), h()))
        .set("width", w())
        .set("height", h())
}
