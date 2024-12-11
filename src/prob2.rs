use z3::{ast, Config, Context, SatResult, Solver};

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

    println!("Solving...");

    if solver.check() == SatResult::Sat {
        println!("---- SAT ----");
        Some(vec![])
    } else {
        println!("---- UNSAT ----");
        None
    }
}

#[cfg(test)]
#[test]
fn test_prob2() {}
