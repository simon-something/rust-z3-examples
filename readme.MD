# Solving constraints with Z3 using Rust

Some random examples of solving constraints with Z3 using Rust, mostly from problem generated by chatGPT or Claude (feel free to open issues with any fun new ones).
Idea of this repo is to improve my knowledge of the z3 crate

## Problem 1 (chatGPT)
"You are organizing a team meeting with three participants: Alice, Bob, and Charlie. You need to satisfy the following constraints:
	1.	If Alice attends the meeting, Bob must attend.
	2.	If Bob attends the meeting, Charlie cannot attend.
	3.	At least one of Alice or Charlie must attend the meeting.

Can you determine whether these constraints can be satisfied, and if so, find a valid arrangement?"

If I'm correct, this is an arity of 2, meaning polynomial time complexity (2-sat).
(A -> B) ^ (B -> !C) ^ (A v C)

## Problem 2 (chatGPT)
"You are organizing a schedule for four meetings—M1, M2, M3, and M4—over two timeslots: Morning (AM) and Afternoon (PM). You have three available conference rooms: R1, R2, and R3. Each meeting must be assigned a single timeslot and exactly one room. The following constraints must all be satisfied simultaneously:
	1.	M1 and M2 cannot be held in the same timeslot.
	2.	If M3 is scheduled in the morning, then M4 cannot be held in R1 or R3.
	3.	If M2 is scheduled in R2, then M1 must not be in the afternoon.
	4.	At least two of the meetings (out of M1, M2, M3, M4) must be in the afternoon timeslot.
	5.	M1 must not be in the same room as M3.
	6.	M4 must be either in a different timeslot than M1 or, if in the same timeslot, it must be in R2.
	7.	If M3 is in R3, then M2 cannot be in the same room as M4.
	8.	If M1 is scheduled in the morning, then M3 must not be scheduled in the afternoon.

Your task is to determine if there is a valid assignment of M1, M2, M3, and M4 to the Morning/Afternoon timeslots and rooms R1, R2, R3 that satisfies all these constraints, and if so, find one such assignment."

## Problem 3 (Einstein, supposedly)
This is the 'classic' Einstein riddle (the original attribution to teenage Einstein or not is outside the scope of z3, iic), with the 5 houses in a row. If you never did it pen&paper, it's really fun!q
"There are five houses, each painted a different color. In each house lives a person with a different nationality.
These five owners drink a certain type of beverage, smoke a certain brand of cigar, and keep a certain pet.
No owners have the same pet, smoke the same cigar, or drink the same beverage.

The question is, who owns the fish?

Here are the clues:

- The Brit lives in the red house
- The Swede keeps dogs as pets
- The Dane drinks tea
- The green house is on the immediate left of the white house
- The green house’s owner drinks coffee
- The person who smokes Pall Mall rears birds
- The owner of the yellow house smokes Dunhill
- The man living in the center house drinks milk
- The Norwegian lives in the first house
- The man who smokes blends lives next to the one who keeps cats
- The man who keeps horses lives next to the man who smokes Dunhill
- The owner who smokes BlueMaster drinks beer
- The German smokes Prince
- The Norwegian lives next to the blue house
- The man who smokes blend has a neighbor who drinks water"

List of every colors-nationality-beverage-cigar-pet:
colors: red, green, yellow, blue, white
nationalities: Brit, Swede, Dane, Norwegian, German
beverages: tea, coffee, milk, beer, water
cigars: Pall Mall, Dunhill, Blend, BlueMaster, Prince
pets: fish, dogs, birds, cats, horses

## Problem 4 (Myself)

```solidity
uint256 public counter = 1;

/// @z3-verify: counter > 0
function increment() public {
    unchecked {
        counter++;
    }
}
```

## Problem 5 (Myself)

```solidity
/// @z3-verify: x > 0
function branching(uint256 x) public {
    if(x == 0) {
        x++;
    } else {
        x--;
    }
}
```
