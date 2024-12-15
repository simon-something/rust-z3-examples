use std::fmt;

use duplicate::duplicate_item;
use z3::{
    ast::{self, Ast, Bool, Int},
    Context, Solver,
};

#[derive(Debug)]
pub struct House<'ctx, 'solver> {
    ctx: &'ctx Context,
    solver: &'solver Solver<'ctx>, // ctx outlives solver
    pub index: Int<'ctx>,
    pub color: Int<'ctx>,
    pub nationality: Int<'ctx>,
    pub beverage: Int<'ctx>,
    pub cigar: Int<'ctx>,
    pub pet: Int<'ctx>,
}

impl<'ctx, 'solver> House<'ctx, 'solver> {
    pub fn new(ctx: &'ctx Context, solver: &'solver Solver<'ctx>, unique_internal_id: i8) -> Self {
        House {
            ctx,
            solver,
            index: ast::Int::new_const(ctx, format!("index_.{}", unique_internal_id)),
            color: ast::Int::new_const(ctx, format!("color_.{}", unique_internal_id)),
            nationality: ast::Int::new_const(ctx, format!("nationality_.{}", unique_internal_id)),
            beverage: ast::Int::new_const(ctx, format!("beverage_.{}", unique_internal_id)),
            cigar: ast::Int::new_const(ctx, format!("cigar_.{}", unique_internal_id)),
            pet: ast::Int::new_const(ctx, format!("pet_.{}", unique_internal_id)),
        }
    }

    // Strong Noir vibe;)
    pub fn serialize(&self) -> Vec<&ast::Int> {
        vec![
            &self.index,
            &self.color,
            &self.nationality,
            &self.beverage,
            &self.cigar,
            &self.pet,
        ]
    }

    /// Add a constraint A => B to the solver
    pub fn constrain_implication(&self, antecedant: &Bool, consequent: &Bool) {
        self.solver.assert(&antecedant.implies(consequent));
    }

    /// Add a constraint index element is +/- 1 index neighbor
    /// @dev do not forget these are constraints, so +/- 1 is done with z3 ast!
    pub fn constrain_next_to(&self, other: &House, element: &Bool, neighbor: &Bool) {
        let one = ast::Int::from_i64(self.ctx, 1);

        self.solver.assert(&(element & neighbor).implies(
            &(self.index._eq(&(&other.index + &one)) | self.index._eq(&(&other.index - &one))),
        ));
    }
}

#[derive(Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    White,
}

#[derive(Clone, Copy)]
pub enum Nationality {
    Brit,
    Swede,
    Dane,
    Norwegian,
    German,
}

#[derive(Clone, Copy)]
pub enum Beverage {
    Tea,
    Coffee,
    Milk,
    Beer,
    Water,
}

#[derive(Clone, Copy)]
pub enum Cigar {
    PallMall,
    Dunhill,
    Blend,
    BlueMaster,
    Prince,
}

#[derive(Clone, Copy)]
pub enum Pet {
    Fish,
    Dogs,
    Birds,
    Cats,
    Horses,
}

// marker trait, for all the enums which are `as i64`-able
trait AsI64 {
    fn as_i64(&self) -> i64;
}

// pretty cool crate tbh
#[duplicate_item(enum_name; [ Color ]; [ Nationality ]; [ Beverage ]; [ Cigar ]; [ Pet ])]
impl AsI64 for enum_name {
    fn as_i64(&self) -> i64 {
        *self as i64
    }
}

pub trait ToZ3Int<'ctx> {
    fn to_z3_int(&self, ctx: &'ctx Context) -> ast::Int<'ctx>;
}

impl<T> ToZ3Int<'_> for T
where
    T: Copy + AsI64,
{
    fn to_z3_int<'ctx>(&self, ctx: &'ctx Context) -> ast::Int<'ctx> {
        ast::Int::from_i64(ctx, self.as_i64())
    }
}

#[duplicate_item(
    enum_name      variant1     variant2    variant3      variant4       variant5;
    [ Color ]      [ Red ]      [ Green ]   [ Yellow ]    [ Blue ]       [ White ];
    [ Nationality] [ Brit ]     [ Swede ]   [ Dane ]      [ Norwegian ]  [ German ];
    [ Beverage ]   [ Tea ]      [ Coffee ]  [ Milk ]      [ Beer ]       [ Water ];
    [ Cigar ]      [ PallMall ] [ Dunhill ] [ Blend ]     [ BlueMaster ] [ Prince ];
    [ Pet ]        [ Fish ]     [ Dogs ]    [ Birds ]     [ Cats ]       [ Horses ]
)]
impl enum_name {
    pub fn from_int(value: i64) -> enum_name {
        match value {
            0 => enum_name::variant1,
            1 => enum_name::variant2,
            2 => enum_name::variant3,
            3 => enum_name::variant4,
            4 => enum_name::variant5,
            _ => unreachable!("z3 constrained this"),
        }
    }
}

#[duplicate_item(
    enum_name      variant1     s1        variant2    s2       variant3   s3      variant4       s4          variant5   s5;
    [ Color ]      [ Red ]      ["Red"]   [ Green ]   ["Green"][ Yellow ] ["Yellow"][ Blue ]     ["Blue"]    [ White ]  ["White"];
    [ Nationality] [ Brit ]     ["Brit"]  [ Swede ]   ["Swede"][ Dane ]   ["Dane"] [ Norwegian ] ["Norwegian"][ German ] ["German"];
    [ Beverage ]   [ Tea ]      ["Tea"]   [ Coffee ]  ["Coffee"][ Milk ]  ["Milk"] [ Beer ]      ["Beer"]    [ Water ]  ["Water"];
    [ Cigar ]      [ PallMall ] ["Pall Mall"][ Dunhill ]["Dunhill"][ Blend ]["Blend"][ BlueMaster ]["Blue Master"][ Prince ]["Prince"];
    [ Pet ]        [ Fish ]     ["Fish"]  [ Dogs ]    ["Dogs"] [ Birds ]  ["Birds"][ Cats ]      ["Cats"]    [ Horses ] ["Horses"]
)]
impl std::fmt::Display for enum_name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            enum_name::variant1 => write!(f, s1),
            enum_name::variant2 => write!(f, s2),
            enum_name::variant3 => write!(f, s3),
            enum_name::variant4 => write!(f, s4),
            enum_name::variant5 => write!(f, s5),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Solution {
    pub color: i64,
    pub nationality: i64,
    pub beverage: i64,
    pub cigar: i64,
    pub pet: i64,
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", Color::from_int(self.color))?;
        writeln!(f, "{}", Nationality::from_int(self.nationality))?;
        writeln!(f, "{}", Beverage::from_int(self.beverage))?;
        writeln!(f, "{}", Cigar::from_int(self.cigar))?;
        writeln!(f, "{}", Pet::from_int(self.pet))
    }
}
