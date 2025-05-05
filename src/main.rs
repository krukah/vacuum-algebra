mod expression;
mod ladder;
mod natural;
mod pair;

fn main() {
    expression::Expression::default()
        .into_iter()
        .map(|exp| (exp, exp.expectation()))
        .filter(|(_, expectation)| expectation != &natural::Natural::zero())
        .for_each(|(exp, expectation)| println!("{} -> {}", exp, expectation));
}
