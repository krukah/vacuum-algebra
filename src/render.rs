use svg::node::element::Line;
use svg::node::element::Rectangle;
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

impl Segment for crate::expression::Expression {
    /// draw color with log scale (in expectation)
    fn scale(&self) -> f32 {
        (self.expectation().size() as f32)
    }

    /// draw thickness with inverse quadratic scale (in depth)
    fn stroke(&self) -> f32 {
        // 4. - ((self.size() as f32) / 32.)
        0.5 + (1. + self.expectation().size() as f32).log2() / 2.
    }

    fn beg(&self) -> (f32, f32) {
        let mut x = 0.5;
        let mut y = 0.5;
        let mut d = 0.5;
        for pair in self
            .to_string()
            .trim()
            .as_bytes()
            .chunks(2)
            .take((self.size() / 2).saturating_sub(1)) // skip the last pair
            .map(std::str::from_utf8)
            .map(Result::unwrap)
        {
            d /= 2.;
            match pair {
                "00" => y -= d, // down
                "01" => x += d, // right
                "10" => y += d, // up
                "11" => x -= d, // left
                "" => break,
                x => unreachable!("invalid pair: {x}"),
            }
        }
        (x, y)
    }

    fn end(&self) -> (f32, f32) {
        let mut x = 0.5;
        let mut y = 0.5;
        let mut d = 0.5;
        for pair in self
            .to_string()
            .trim()
            .as_bytes()
            .chunks(2)
            .map(std::str::from_utf8)
            .map(Result::unwrap)
        {
            d /= 2.;
            match pair {
                "00" => y -= d, // down
                "01" => x += d, // right
                "10" => y += d, // up
                "11" => x -= d, // left
                "" => break,
                x => unreachable!("invalid pair: {x}"),
            }
        }
        (x, y)
    }
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
    // Convert scale to a color (blue to red gradient)
    let scale = line.scale().min(1.0).max(0.0); // Ensure scale is between 0.0 and 1.0
    let r = (scale * 255.0) as u8;
    let g = 0;
    let b = ((1.0 - scale) * 255.0) as u8;
    let color = format!("rgb({},{},{})", r, g, b);
    Line::new()
        .set("x1", line.beg().0 * w() as f32)
        .set("y1", line.beg().1 * h() as f32)
        .set("x2", line.end().0 * w() as f32)
        .set("y2", line.end().1 * h() as f32)
        .set("stroke", color)
        .set("stroke-width", line.stroke())
}

fn canvas() -> Document {
    // Create rectangle for bounding box
    let bounds = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", w())
        .set("height", h())
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1);
    // Create x-axis (horizontal line through middle)
    let x_axis = Line::new()
        .set("x1", 0)
        .set("y1", h() / 2)
        .set("x2", w())
        .set("y2", h() / 2)
        .set("stroke", "gray")
        .set("stroke-width", 1)
        .set("stroke-dasharray", "4 4");
    // Create y-axis (vertical line through middle)
    let y_axis = Line::new()
        .set("x1", w() / 2)
        .set("y1", 0)
        .set("x2", w() / 2)
        .set("y2", h())
        .set("stroke", "gray")
        .set("stroke-width", 1)
        .set("stroke-dasharray", "4 4");
    Document::new()
        .set("viewBox", (0, 0, w(), h()))
        .set("width", w())
        .set("height", h())
        .add(bounds)
        .add(x_axis)
        .add(y_axis)
}
