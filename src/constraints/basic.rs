use z3::{
    ast::{Ast, Int},
    Context, Solver,
};

use crate::definitions::{
    series::Series,
    series_slot::{ALL_STAR_SERIES_SLOT, FIRST_SERIES_SLOT, LAST_SERIES_SLOT},
    team::Team,
    variables::SATVariables,
};

pub fn assert_basic_constraints<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
    teams: &Vec<Team>,
    first_half: bool,
) {
    assert_no_series_conflict_dates(solver, vars, series, teams);
    assert_series_begin_and_end_dates(context, solver, vars, series, first_half);
}

fn assert_no_series_conflict_dates<'a>(
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
    teams: &Vec<Team>,
) {
    for &team in teams {
        let series_with_this_team: Vec<&Series> = series
            .iter()
            .filter(|s| s.home_team == team || s.away_team == team)
            .collect();

        for (series_1_index, &series_1) in series_with_this_team.iter().enumerate() {
            let &series_1_var_idx = vars.slots_of_series_starts.get(&series_1).unwrap();
            let series_1_var = vars.int_vars.get(series_1_var_idx).unwrap();

            for (series_2_index, &series_2) in series_with_this_team.iter().enumerate() {
                if series_1_index == series_2_index {
                    continue;
                }

                let &series_2_var_idx = vars.slots_of_series_starts.get(&series_2).unwrap();
                let series_2_var = vars.int_vars.get(series_2_var_idx).unwrap();

                solver.assert(&series_1_var._eq(series_2_var).not());
            }
        }
    }
}

fn assert_series_begin_and_end_dates<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
    first_half: bool,
) {
    let first_allowed_slot = if first_half {
        FIRST_SERIES_SLOT
    } else {
        ALL_STAR_SERIES_SLOT + 1
    } as i64;
    let last_allowed_slot = if first_half {
        ALL_STAR_SERIES_SLOT - 1
    } else {
        LAST_SERIES_SLOT
    } as i64;

    for &series in series {
        let &series_var_idx = vars.slots_of_series_starts.get(&series).unwrap();
        let series_var = vars.int_vars.get(series_var_idx).unwrap();
        solver.assert(&series_var.ge(&Int::from_i64(context, first_allowed_slot as i64)));
        solver.assert(&series_var.le(&Int::from_i64(context, last_allowed_slot as i64)));
    }
}
