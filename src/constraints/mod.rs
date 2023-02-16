use z3::{Context, Solver};

use crate::definitions::team::Team;
use crate::definitions::{series::Series, variables::SATVariables};

use self::adjacency::assert_adjacency_constraints;
use self::basic::assert_basic_constraints;
use self::distributional::assert_distributional_constraints;
use self::special_dates::assert_special_dates_constraints;

mod adjacency;
mod basic;
mod distributional;
mod special_dates;

pub fn assert_all_constraints<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
    teams: &Vec<Team>,
    first_half: bool,
) {
    assert_basic_constraints(context, solver, vars, series, teams, first_half);
    assert_adjacency_constraints(context, solver, vars, series, first_half);
    // assert_distributional_constraints(context, solver, vars, series);
    // assert_special_dates_constraints(context, solver, vars, series);
}
