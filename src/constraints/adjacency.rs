use std::collections::HashMap;

use z3::{
    ast::{Ast, Bool},
    Context, Solver,
};

use crate::definitions::{
    series::Series,
    series_slot::{ALL_STAR_SERIES_SLOT, FIRST_SERIES_SLOT, LAST_SERIES_SLOT},
    team::{Team, TEAMS},
    variables::SATVariables,
};

pub fn assert_adjacency_constraints<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
    first_half: bool,
) {
    assert_no_adjacent_series_between_same_teams(solver, vars, series);
    assert_no_one_series_homestands_or_roadtrips(context, solver, vars, first_half);
    assert_no_four_series_homestands_or_roadtrips(context, solver, vars, first_half);
}

fn assert_no_adjacent_series_between_same_teams<'a>(
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
) {
    let mut team_pair_to_series: HashMap<(Team, Team), Vec<Series>> = HashMap::new();

    for series in series {
        let home_team = series.home_team;
        let away_team = series.away_team;

        let pair = if home_team < away_team {
            (home_team, away_team)
        } else {
            (away_team, home_team)
        };

        if team_pair_to_series.contains_key(&pair) {
            team_pair_to_series.get_mut(&pair).unwrap().push(*series);
        } else {
            team_pair_to_series.insert(pair, vec![*series]);
        }
    }

    for series_list in team_pair_to_series.values() {
        let num_series = series_list.len();
        for (first_series_index, first_series) in series_list.iter().enumerate() {
            let first_slot = vars.slots_of_series_starts.get(first_series).unwrap();
            let first_slot_var = vars.int_vars.get(*first_slot).unwrap();

            for second_series_index in first_series_index + 1..num_series {
                let second_series = series_list.get(second_series_index).unwrap();
                let second_slot = vars.slots_of_series_starts.get(second_series).unwrap();
                let second_slot_var = vars.int_vars.get(*second_slot).unwrap();

                solver.assert(&first_slot_var._eq(second_slot_var).not());
            }
        }
    }
}

fn assert_no_one_series_homestands_or_roadtrips<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    first_half: bool,
) {
    // ignore edge cases, that's what they're for
    let first_series = if first_half {
        FIRST_SERIES_SLOT + 1
    } else {
        ALL_STAR_SERIES_SLOT + 2
    };
    let last_series = if first_half {
        ALL_STAR_SERIES_SLOT - 2
    } else {
        LAST_SERIES_SLOT - 1
    };

    // No one-series homestands or roadtrips
    for &team in TEAMS.iter() {
        for slot in first_series..=last_series {
            let [home_prev, home_this, home_next] = [slot - 1, slot, slot + 1].map(|s| {
                vars.bool_vars
                    .get(*vars.plays_at_home.get(&(team, s)).unwrap())
                    .unwrap()
            });

            let [away_prev, away_this, away_next] = [slot - 1, slot, slot + 1].map(|s| {
                vars.bool_vars
                    .get(*vars.plays_away.get(&(team, s)).unwrap())
                    .unwrap()
            });

            solver.assert(&home_this.implies(&Bool::or(context, &[home_prev, home_next])));
            solver.assert(&away_this.implies(&Bool::or(context, &[away_prev, away_next])));
        }
    }
}

fn assert_no_four_series_homestands_or_roadtrips<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    first_half: bool,
) {
    // Unfortunately the bounds are slightly different, so we can't reuse the
    // loop from the no-singletons assertion
    let first_series = if first_half {
        FIRST_SERIES_SLOT
    } else {
        ALL_STAR_SERIES_SLOT + 1
    };
    let last_series = if first_half {
        ALL_STAR_SERIES_SLOT - 4
    } else {
        LAST_SERIES_SLOT - 3
    };

    for &team in TEAMS.iter() {
        for slot in first_series..=last_series {
            let four_at_home = [slot, slot + 1, slot + 2, slot + 3].map(|s| {
                vars.bool_vars
                    .get(*vars.plays_at_home.get(&(team, s)).unwrap())
                    .unwrap()
            });

            let four_away = [slot, slot + 1, slot + 2, slot + 3].map(|s| {
                vars.bool_vars
                    .get(*vars.plays_away.get(&(team, s)).unwrap())
                    .unwrap()
            });

            solver.assert(&Bool::and(context, &four_at_home).not());
            solver.assert(&Bool::and(context, &four_away).not());
        }
    }
}
