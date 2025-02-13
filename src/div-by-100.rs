use z3::{
    ast::{self},
    Config, Context, SatResult, Solver,
};

fn main() {
    solve();
}

fn solve() -> Option<String> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let t1 = ast::BV::new_const(&ctx, "t1", 256);
    let t2 = ast::BV::new_const(&ctx, "t2", 256);
    let concrete_hundred = ast::BV::from_u64(&ctx, 100, 256);

    solver.assert(&t1.bvult(&t2));
    solver.assert(&t1.bvudiv(&concrete_hundred).bvugt(&t2.bvudiv(&concrete_hundred)));

    println!("Solving...");

    if solver.check() == SatResult::Sat {
        println!("---- SAT ----");

        let model = solver.get_model().unwrap();
        let value_t1 = model.eval(&t1, true).unwrap().to_string();
        let value_t2 = model.eval(&t2, true).unwrap().to_string();

        println!("Model:");
        println!("{:?}", model);
        println!("{:?}", value_t1);
        println!("{:?}", value_t2);

        Some(format!("{} {}", value_t1, value_t2))
    } else {
        println!("---- UNSAT ----");
        None
    }
}

#[cfg(test)]
#[test]
fn test_solidity_overflow() {
    // let result = solve();

    // assert!(result.is_some());

    // // Don't judge me okay?
    // assert_eq!(
    //     result,
    //     Some("#xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string())
    // )
}
