use vacuum::*;

fn main() {
    crate::expression::Expression::default()
        .into_iter()
        .map(|expression| (expression, expression.expectation().size()))
        .filter(|(_, expectation)| expectation != &0)
        .map(|(expression, size)| (expression.to_string(), size))
        .collect::<std::collections::BTreeMap<String, usize>>()
        .iter()
        .for_each(|(expression, expectation)| println!("{} -> {:>16}", expression, expectation));
}
