use z3::{ast, Config, Context, SatResult, Solver};

fn main() {
    println!(
        "Problem 1:\n
    1.	If Alice attends the meeting, Bob must attend.\n
	2.	If Bob attends the meeting, Charlie cannot attend.\n
	3.	At least one of Alice or Charlie must attend the meeting.\n"
    );

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let alice = ast::Bool::new_const(&ctx, "alice");
    let bob = ast::Bool::new_const(&ctx, "bob");
    let charlie = ast::Bool::new_const(&ctx, "charlie");

    let first_constraint = alice.implies(&bob);
    let second_constraint = bob.implies(&charlie.not());
    let third_constraint = ast::Bool::or(&ctx, &[&alice, &charlie]);

    solver.assert(&first_constraint);
    solver.assert(&second_constraint);
    solver.assert(&third_constraint);

    println!("Solving...");

    if solver.check() == SatResult::Sat {
        println!("---- SAT ----");

        let model = solver.get_model().unwrap();

        println!("Model:");
        format!(
            "
            Alice: {},
            Bob: {},
            Charlie: {}",
            model.eval(&alice, false).unwrap(),
            model.eval(&bob, false).unwrap(),
            model.eval(&charlie, false).unwrap(),
        );
    } else {
        println!("---- UNSAT ----");
    }
}
