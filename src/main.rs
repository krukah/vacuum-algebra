use vacuum::*;

fn main() {
    crate::render::draw(
        crate::expression::Expression::default()
            .into_iter()
            .take(0x10000)
            .map(|expr| (expr, expr.expectation().size()))
            .filter(|(_, value)| value != &0)
            .inspect(|(expr, value)| println!("{} -> {:>16}", expr, value))
            .map(|(expr, _)| expr),
    )
    .expect("picture to save");
}
