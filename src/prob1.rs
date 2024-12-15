use z3::{ast, Config, Context, SatResult, Solver};

fn main() {
    println!(
        "Problem 1:
        1. If Alice attends the meeting, Bob must attend.
    	2. If Bob attends the meeting, Charlie cannot attend.
    	3. At least one of Alice or Charlie must attend the meeting."
    );

    solve();
}

fn solve() -> Option<Vec<bool>> {
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
        let value = vec![
            model.eval(&alice, true).unwrap().as_bool().unwrap(),
            model.eval(&bob, true).unwrap().as_bool().unwrap(),
            model.eval(&charlie, true).unwrap().as_bool().unwrap(),
        ];

        println!("Model:");
        println!("Alice: {}", value[0]);
        println!("Bob: {}", value[1]);
        println!("Charlie: {}", value[2]);

        Some(value)
    } else {
        println!("---- UNSAT ----");
        None
    }
}

#[cfg(test)]
// Possible solution: Alice and Bob attend the meeting, Charlie does not attend.
// True - True - False
#[test]
fn test_meeting() {
    let values = solve();

    assert!(values.is_some());
    let values = values.unwrap();

    assert!(values[0]);
    assert!(values[1]);
    assert!(!values[2]);
}
