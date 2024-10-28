mod calculator;

fn main() {
    let tokens = calculator::Calculator::parse("2 *(2 + 48) / 4");
    let expr = calculator::Calculator::expression(tokens.unwrap());
    println!("{:?}",expr);
    let res = calculator::Calculator::evaluate(expr);
    println!("{:?}", res.unwrap())
}
