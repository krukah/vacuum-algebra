mod expression;
mod ladder;
mod natural;
mod pair;

fn main() {
    expression::Expression::default()
        .into_iter()
        .map(|exp| (exp, exp.expectation().size()))
        .filter(|(_, expectation)| expectation != &0)
        .for_each(|(exp, expectation)| println!("{} -> {:>16}", exp, expectation));
}
