mod types;

use types::types_prob3::{
    Beverage::*, Cigar::*, Color::*, House, Nationality::*, Pet::*, Solution, ToZ3Int,
};
use z3::{
    ast::{self, Ast},
    Config, Context, SatResult, Solver,
};

fn main() {
    println!(
        "
        The question is, who owns the fish?

        The Brit lives in the red house
        The Swede keeps dogs as pets
        The Dane drinks tea
        The green house is on the left of the white house
        The green house’s owner drinks coffee
        The person who smokes Pall Mall rears birds
        The owner of the yellow house smokes Dunhill
        The man living in the center house drinks milk
        The Norwegian lives in the first house
        The man who smokes blends lives next to the one who keeps cats
        The man who keeps horses lives next to the man who smokes Dunhill
        The owner who smokes BlueMaster drinks beer
        The German smokes Prince
        The Norwegian lives next to the blue house
        The man who smokes blend has a neighbor who drinks water

        List of every colors-nationality-beverage-cigar-pet:
        colors: red, green, yellow, blue, white
        nationalities: Brit, Swede, Dane, Norwegian, German
        beverages: tea, coffee, milk, beer, water
        cigars: Pall Mall, Dunhill, Blend, BlueMaster, Prince
        pets: fish, dogs, birds, cats, horses
        "
    );

    solve();
}

fn solve() -> Option<Vec<Solution>> {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // We solve by house, which are gather in a street (I know, don't judge me)
    let street: [House; 5] = array_init::array_init(|i: usize| House::new(&ctx, &solver, i as i8));

    // Main loop, apply constraints on each house
    for (i, house) in street.iter().enumerate() {
        // Each characteristic is unique (one fish, one milk, etc)
        // Skippy made me do the skip:
        for other_house in street.iter().skip(i + 1) {
            let fields_of_i = house.serialize();
            let fields_of_j = other_house.serialize();

            for (field_i, field_j) in fields_of_i.iter().zip(fields_of_j.iter()) {
                solver.assert(&!(&field_i._eq(field_j)));
            }
        }

        // Each entries is bounded [0, 4]
        house.serialize().iter().for_each(|x| {
            solver.assert(&x.ge(&ast::Int::from_u64(&ctx, 0)));
            solver.assert(&x.lt(&ast::Int::from_u64(&ctx, 5)));
        });

        // brit in red house
        house.constrain_implication(
            &house.nationality._eq(&Brit.to_z3_int(&ctx)),
            &house.color._eq(&ast::Int::from_i64(&ctx, Red as i64)),
        );

        // Swede keeps dogs
        house.constrain_implication(
            &house.nationality._eq(&Swede.to_z3_int(&ctx)),
            &house.pet._eq(&Dogs.to_z3_int(&ctx)),
        );

        // Dane Tea
        house.constrain_implication(
            &house.nationality._eq(&Dane.to_z3_int(&ctx)),
            &house.beverage._eq(&Tea.to_z3_int(&ctx)),
        );

        // Green on left of white
        // We want index green -1 == index white -> we iterate all the other house,
        // as the street array *isn't sorted yet* (hence the index field for House)
        for (j, other_house) in street.iter().enumerate() {
            if i == j {
                continue;
            }

            solver.assert(
                &(&house.color._eq(&Green.to_z3_int(&ctx))
                    & &other_house.color._eq(&White.to_z3_int(&ctx)))
                    .implies(
                        &house
                            .index
                            ._eq(&(&other_house.index - &ast::Int::from_i64(&ctx, 1))),
                    ),
            )
        }

        // Green coffee
        house.constrain_implication(
            &house.color._eq(&Green.to_z3_int(&ctx)),
            &house.beverage._eq(&Coffee.to_z3_int(&ctx)),
        );

        // Pall Mall birds
        house.constrain_implication(
            &house.cigar._eq(&PallMall.to_z3_int(&ctx)),
            &house.pet._eq(&Birds.to_z3_int(&ctx)),
        );

        // Yellow Dunhill
        house.constrain_implication(
            &house.color._eq(&Yellow.to_z3_int(&ctx)),
            &house.cigar._eq(&Dunhill.to_z3_int(&ctx)),
        );

        // Center house milk
        house.constrain_implication(
            &house.index._eq(&ast::Int::from_i64(&ctx, 2)),
            &house.beverage._eq(&Milk.to_z3_int(&ctx)),
        );

        // Norwegian first house
        house.constrain_implication(
            &house.nationality._eq(&Norwegian.to_z3_int(&ctx)),
            &house.index._eq(&ast::Int::from_i64(&ctx, 0)),
        );

        // blends next to cats
        for (j, other_house) in street.iter().enumerate() {
            // doing this with a constraint "i.index != j.index" instead
            // doesn't work, as it constrain "i.index != i.index" too
            // (trivial unsat) + we constrain index uniqueness for different
            // houses
            if i == j {
                continue;
            }

            // "next to" exclude "in same house"
            solver.assert(
                &!(&house.cigar._eq(&Blend.to_z3_int(&ctx))
                    & &house.pet._eq(&Cats.to_z3_int(&ctx))),
            );

            house.constrain_next_to(
                other_house,
                &house.cigar._eq(&Blend.to_z3_int(&ctx)),
                &other_house.pet._eq(&Cats.to_z3_int(&ctx)),
            );
        }

        // Horses next to dunhill
        for (j, other_house) in street.iter().enumerate() {
            if i == j {
                continue;
            }

            house.constrain_next_to(
                other_house,
                &house.pet._eq(&Horses.to_z3_int(&ctx)),
                &other_house.cigar._eq(&Dunhill.to_z3_int(&ctx)),
            );
        }

        // Bluemaster drinks beer
        house.constrain_implication(
            &house.cigar._eq(&BlueMaster.to_z3_int(&ctx)),
            &house.beverage._eq(&Beer.to_z3_int(&ctx)),
        );

        // German smokes prince
        house.constrain_implication(
            &house.nationality._eq(&German.to_z3_int(&ctx)),
            &house.cigar._eq(&Prince.to_z3_int(&ctx)),
        );

        // Norwegian next to blue house
        for (j, other_house) in street.iter().enumerate() {
            if i == j {
                continue;
            }

            solver.assert(
                &!(&house.nationality._eq(&Norwegian.to_z3_int(&ctx))
                    & &house.color._eq(&Blue.to_z3_int(&ctx))),
            );

            house.constrain_next_to(
                other_house,
                &house.nationality._eq(&Norwegian.to_z3_int(&ctx)),
                &other_house.color._eq(&Blue.to_z3_int(&ctx)),
            );
        }

        // Blend next to water
        for (j, other_house) in street.iter().enumerate() {
            if i == j {
                continue;
            }

            solver.assert(
                &!(&house.cigar._eq(&Blend.to_z3_int(&ctx))
                    & &house.beverage._eq(&Water.to_z3_int(&ctx))),
            );

            house.constrain_next_to(
                other_house,
                &house.cigar._eq(&Blend.to_z3_int(&ctx)),
                &other_house.beverage._eq(&Water.to_z3_int(&ctx)),
            );
        }
    }

    println!("Solving...");

    if solver.check() == SatResult::Sat {
        println!("---- SAT ----");

        let model = solver.get_model().unwrap();

        println!("Model:");

        let mut solutions: Vec<_> = street
            .iter()
            .map(|house| {
                (
                    model.eval(&house.index, true).unwrap().as_i64().unwrap(),
                    Solution {
                        color: model.eval(&house.color, true).unwrap().as_i64().unwrap(),
                        nationality: model
                            .eval(&house.nationality, true)
                            .unwrap()
                            .as_i64()
                            .unwrap(),
                        beverage: model.eval(&house.beverage, true).unwrap().as_i64().unwrap(),
                        cigar: model.eval(&house.cigar, true).unwrap().as_i64().unwrap(),
                        pet: model.eval(&house.pet, true).unwrap().as_i64().unwrap(),
                    },
                )
            })
            .collect();
        solutions.sort_by_key(|&(index, _)| index);

        let solutions: Vec<Solution> = solutions.into_iter().map(|(_, sol)| sol).collect();

        for i in &solutions {
            println!("-----------");
            println!("{}", i);
        }

        Some(solutions)
    } else {
        println!("---- UNSAT ----");
        None
    }
}

#[cfg(test)]
#[test]
fn test_einstein() {
    let result = solve();

    assert!(result.is_some());

    let solution = [
        Solution {
            color: Yellow as i64,
            nationality: Norwegian as i64,
            beverage: Water as i64,
            cigar: Dunhill as i64,
            pet: Cats as i64,
        },
        Solution {
            color: Blue as i64,
            nationality: Dane as i64,
            beverage: Tea as i64,
            cigar: Blend as i64,
            pet: Horses as i64,
        },
        Solution {
            color: Red as i64,
            nationality: Brit as i64,
            beverage: Milk as i64,
            cigar: PallMall as i64,
            pet: Birds as i64,
        },
        Solution {
            color: Green as i64,
            nationality: German as i64,
            beverage: Coffee as i64,
            cigar: Prince as i64,
            pet: Fish as i64,
        },
        Solution {
            color: White as i64,
            nationality: Swede as i64,
            beverage: Beer as i64,
            cigar: BlueMaster as i64,
            pet: Dogs as i64,
        },
    ];

    for (i, house) in result.unwrap().into_iter().enumerate() {
        assert_eq!(house, solution[i]);
    }

    // house1 yellow norwegian water dunhill cat
    // 2 blue dane tea blend horse
    // 3 red brit milk pall mall birds
    // 4 green german coffee prince FISH
    // 5 white swede beer bluemasters dog
}
