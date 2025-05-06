use vacuum::*;

fn main() {
    crate::render::draw(
        crate::expression::Expression::default()
            .into_iter()
            .take(0xF000)
            .map(|expr| (expr, expr.expectation().size()))
            .inspect(|(expr, value)| println!("{} -> {:>16}", expr, value))
            .map(|(expr, _)| expr),
    )
    .expect("picture to save");
}
