mod types;

use z3::{
    ast::{self, Ast, BV},
    Config, Context, SatResult, Solver,
};

struct Solution {}

fn main() {
    println!(
        "
        ```solidity
        uint256 public counter = 1;

        /// @z3-verify: counter > 0
        function increment() public {{
            unchecked {{
                counter++;
            }}
        }}
        ```
        "
    );

    solve();
}

fn increment<'a>(x: &'a BV) -> BV<'a> {
    let one = ast::BV::from_i64(x.get_ctx(), 1, 256);
    x.bvadd(&one)
}

fn solve() -> Option<String> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // We'll go first by building a cfg for the function, with a symbolic representation
    // of the counter variable and a single operation (incr).
    // we use bv256 to keep the solidity type

    let counter = ast::BV::new_const(&ctx, "counter", 256);

    // As we use `unchecked`, we should get type(uint256).max as value satisfying this constraint
    solver.assert(&increment(&counter)._eq(&ast::BV::from_i64(&ctx, 0, 256)));

    println!("Solving...");

    if solver.check() == SatResult::Sat {
        println!("---- SAT ----");

        let model = solver.get_model().unwrap();
        let value = model.eval(&counter, true).unwrap().to_string();

        println!("Model:");
        println!("{:?}", model);
        println!("{:?}", value);

        Some(value)
    } else {
        println!("---- UNSAT ----");
        None
    }
}

#[cfg(test)]
#[test]
fn test_solidity_overflow() {
    let result = solve();

    assert!(result.is_some());

    // Don't judge me okay?
    assert_eq!(
        result,
        Some("#xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string())
    )
}
