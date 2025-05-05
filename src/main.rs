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

    // case studies

    // never returns
    // 0000000001101110111001111110010101110111100010000011010011111111
    // ________________________________00011110100001111100101101101111

    // returns 2
    // ________________________________________________________01001101

    // returns 12
    // ________________________________________________________00100111
}
