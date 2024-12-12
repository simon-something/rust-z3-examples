use z3::{
    ast::{self, Ast},
    Config, Context, SatResult, Solver,
};

fn main() {
    println!(
        "You are organizing a schedule for four meetings—M1, M2, M3, and M4—over two timeslots: Morning (AM) and Afternoon (PM).
        You have three available conference rooms: R1, R2, and R3.
        Each meeting must be assigned a single timeslot and exactly one room.
        The following constraints must all be satisfied simultaneously:
	1.	M1 and M2 cannot be held in the same timeslot.
	2.	If M3 is scheduled in the morning, then M4 cannot be held in R1 or R3.
	3.	If M2 is scheduled in R2, then M1 must not be in the afternoon.
	4.	At least two of the meetings (out of M1, M2, M3, M4) must be in the afternoon timeslot.
	5.	M1 must not be in the same room as M3.
	6.	M4 must be either in a different timeslot than M1 or, if in the same timeslot, it must be in R2.
	7.	If M3 is in R3, then M2 cannot be in the same room as M4.
	8.	If M1 is scheduled in the morning, then M3 must not be scheduled in the afternoon.

        Your task is to determine if there is a valid assignment of M1, M2, M3, and M4 to the Morning/Afternoon timeslots and rooms R1, R2, R3 that satisfies all these constraints, and if so, find one such assignment."

    );

    solve();
}

fn solve() -> Option<Vec<bool>> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // am == is AM?
    let m1_am = ast::Bool::new_const(&ctx, "m1_am");
    let m1_room = ast::Int::new_const(&ctx, "m1_room");
    let m2_am = ast::Bool::new_const(&ctx, "m2_am");
    let m2_room = ast::Int::new_const(&ctx, "m2_room");
    let m3_am = ast::Bool::new_const(&ctx, "m3_am");
    let m3_room = ast::Int::new_const(&ctx, "m3_room");
    let m4_am = ast::Bool::new_const(&ctx, "m4_am");
    let m4_room = ast::Int::new_const(&ctx, "m4_room");

    // Rooms need to be in [1, 3]
    for room in [&m1_room, &m2_room, &m3_room, &m4_room] {
        solver.assert(&room.ge(&ast::Int::from_i64(&ctx, 1)));
        solver.assert(&room.le(&ast::Int::from_i64(&ctx, 3)));
    }

    // No duplicates (cannot have same meeting AND same room)
    solver.assert(&!((&m1_room._eq(&m2_room)) & (&m1_am._eq(&m2_am))));
    solver.assert(&!((&m1_room._eq(&m3_room)) & (&m1_am._eq(&m3_am))));
    solver.assert(&!((&m1_room._eq(&m4_room)) & (&m1_am._eq(&m4_am))));
    solver.assert(&!((&m2_room._eq(&m3_room)) & (&m2_am._eq(&m3_am))));
    solver.assert(&!((&m2_room._eq(&m4_room)) & (&m2_am._eq(&m4_am))));
    solver.assert(&!((&m3_room._eq(&m4_room)) & (&m3_am._eq(&m4_am))));

    // 1.	M1 and M2 cannot be held in the same timeslot.
    solver.assert(&ast::Bool::xor(&m1_am, &m2_am));

    // 2.	If M3 is scheduled in the morning, then M4 cannot be held in R1 or R3.
    solver.assert(&m3_am.implies(&m4_room._eq(&ast::Int::from_i64(&ctx, 2))));

    // 3.	If M2 is scheduled in R2, then M1 must not be in the afternoon.
    solver.assert(&m2_room._eq(&ast::Int::from_i64(&ctx, 2)).implies(&m1_am));

    // 4.   At least two of the meetings (out of M1, M2, M3, M4) must be in the afternoon timeslot.
    solver.assert(
        &ast::Int::add(
            &ctx,
            &[
                &m1_am.ite(&ast::Int::from_i64(&ctx, 1), &ast::Int::from_i64(&ctx, 0)),
                &m2_am.ite(&ast::Int::from_i64(&ctx, 1), &ast::Int::from_i64(&ctx, 0)),
                &m3_am.ite(&ast::Int::from_i64(&ctx, 1), &ast::Int::from_i64(&ctx, 0)),
                &m4_am.ite(&ast::Int::from_i64(&ctx, 1), &ast::Int::from_i64(&ctx, 0)),
            ],
        )
        .ge(&ast::Int::from_i64(&ctx, 2)),
    );

    // 5.	M1 must not be in the same room as M3.
    solver.assert(&m1_room._eq(&m3_room).not());

    // 6.	M4 must be either in a different timeslot than M1 or, if in the same timeslot, it must be in R2.
    solver.assert(&(m4_am._eq(&m1_am)).implies(&m4_room._eq(&ast::Int::from_i64(&ctx, 2))));

    // 7.	If M3 is in R3, then M2 cannot be in the same room as M4.
    solver.assert(&(m3_room._eq(&ast::Int::from_i64(&ctx, 3))).implies(&m2_room._eq(&m4_room)));

    // 8.	If M1 is scheduled in the morning, then M3 must not be scheduled in the afternoon.
    solver.assert(&m1_am.implies(&m3_am));

    println!("Solving...");

    if solver.check() == SatResult::Sat {
        println!("---- SAT ----");

        let model = solver.get_model().unwrap();
        let value_am = vec![
            model.eval(&m1_am, false).unwrap().as_bool().unwrap(),
            model.eval(&m2_am, false).unwrap().as_bool().unwrap(),
            model.eval(&m3_am, false).unwrap().as_bool().unwrap(),
            model.eval(&m4_am, false).unwrap().as_bool().unwrap(),
        ];

        let value_room = vec![
            model.eval(&m1_room, false).unwrap(),
            model.eval(&m2_room, false).unwrap(),
            model.eval(&m3_room, false).unwrap(),
            model.eval(&m4_room, false).unwrap(),
        ];

        println!("Model:");
        println!("M1: AM: {}, Room: {}", value_am[0], value_room[0]);
        println!("M2: AM: {}, Room: {}", value_am[1], value_room[1]);
        println!("M3: AM: {}, Room: {}", value_am[2], value_room[2]);
        println!("M4: AM: {}, Room: {}", value_am[3], value_room[3]);

        Some(vec![])
    } else {
        println!("---- UNSAT ----");
        None
    }
}

#[cfg(test)]
#[test]
fn test_prob2() {}
