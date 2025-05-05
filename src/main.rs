mod expression;
mod ladder;
mod natural;
mod pair;

fn main() {
    use expression::Expression;
    use rand;

    let expression = Expression::from((rand::random::<u8>() as u64, 8));
    println!("expression: {}", expression);
    println!("evaluation: {}", expression.expectation());
}
