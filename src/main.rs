mod evaluator;

use evaluator::{
    builtins::InfixOperators,
    control::Control,
    operations::{Func, OperationType},
    Evaluator, Token,
};

fn main() {
    use Control::*;
    use Token::*;

    let mut eval = Evaluator::new();

    eval.add_fn(
        "Print".into(),
        1,
        Box::new(|x| {
            println!("{}", x[0]);
            x[0]
        }),
    );

    let tokens = vec![
        Operation(OperationType::Func(Func::FunStart("Print"))),
        Number(7.),
        Operation(OperationType::Func(Func::FunEnd)),
        Operation(OperationType::BuiltinFun(InfixOperators::Mul)),
        Number(6.),
        Operation(OperationType::Control(EndExpr)),
    ];

    // eval.eval(&tokens).unwrap();
    println!("{:?} = {:?}", tokens, eval.eval(&tokens));
}
