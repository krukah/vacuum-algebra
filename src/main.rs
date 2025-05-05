pub mod expression;
pub mod ladder;
pub mod natural;
pub mod pair;

fn main() {
    use expression::Expression;
    use rand;
    // 0000000001101110111001111110010101110111100010000011010011111111
    // this value hangs, worth investigating
    let expression = Expression::from((rand::random::<u64>(), 64));
    println!("expression: {}", expression);
    println!("evaluation: {}", expression.expectation());
}
