use z3::{
    ast::{Ast, Int},
    Context, Solver,
};

use crate::definitions::{
    series::Series, series_slot::ALL_STAR_SERIES_SLOT, variables::SATVariables,
};

pub fn assert_special_dates_constraints<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
) {
    // assert_blue_jays_at_home_on_canada_day(solver, vars);
}

/*
fn assert_blue_jays_at_home_on_canada_day<'a>(solver: &mut Solver, vars: &SATVariables) {
    let canada_day = date_from_month_and_day(Month::July, 1).unwrap();

    let var_index = vars
        .team_plays_at_home
        .get(&(Team::BlueJays, canada_day))
        .unwrap();

    solver.assert(vars.bool_vars.get(*var_index).unwrap());
}
*/
