use std::collections::HashMap;

use z3::ast::{Ast, Bool, Int};
use z3::Context;

use super::division::Division;
use super::series::Series;
use super::series_slot::{SeriesSlot, ALL_STAR_SERIES_SLOT, FIRST_SERIES_SLOT, LAST_SERIES_SLOT};
use super::team::{Team, TEAMS};

pub struct SATVariables<'a> {
    // THE SOURCE OF TRUTH
    pub slots_of_series_starts: HashMap<Series, usize>,

    // values in slots_of_series_starts are indices into int_vars
    pub int_vars: Vec<Int<'a>>,

    // helper indicator variables
    pub plays_at_home: HashMap<(Team, SeriesSlot), usize>,
    pub plays_away: HashMap<(Team, SeriesSlot), usize>,
    pub plays_away_west: HashMap<(Team, SeriesSlot), usize>,
    pub plays_away_central: HashMap<(Team, SeriesSlot), usize>,
    pub plays_away_east: HashMap<(Team, SeriesSlot), usize>,

    // values in those hashmaps are indices into bool_vars
    pub bool_vars: Vec<Bool<'a>>,
}

impl<'a> SATVariables<'_> {
    pub fn new(context: &'a Context, serieses: &Vec<Series>, first_half: bool) -> SATVariables<'a> {
        let mut vars = SATVariables {
            slots_of_series_starts: HashMap::new(),
            int_vars: Vec::new(),

            plays_at_home: HashMap::new(),
            plays_away: HashMap::new(),
            plays_away_west: HashMap::new(),
            plays_away_central: HashMap::new(),
            plays_away_east: HashMap::new(),
            bool_vars: Vec::new(),
        };

        // spare me grammar
        for &series in serieses {
            let int_var = Int::new_const(context, format!("slot of {:?}", series));
            vars.int_vars.push(int_var);
            let var_index = vars.int_vars.len() - 1;
            vars.slots_of_series_starts.insert(series, var_index);
        }

        for &team in TEAMS.iter() {
            let first_slot = if first_half {
                FIRST_SERIES_SLOT
            } else {
                ALL_STAR_SERIES_SLOT + 1
            };
            let last_slot = if first_half {
                ALL_STAR_SERIES_SLOT - 1
            } else {
                LAST_SERIES_SLOT
            };

            for slot in first_slot..=last_slot {
                // This is barbaric but necessary?, given that we can only do
                // ORs of a fixed number of variables: for _every team_, check
                // whether _any series_ (many of which can't exist!) overlaps,
                // substituting Falses in the big OR statement for when the
                // series can't exist - this is 90 variables, many of which are
                // fake
                let plays_at_home_vars = TEAMS
                    .iter()
                    .map(|&other_team| {
                        [2, 3, 4].map(|length| {
                            if let Some(&idx) = vars.slots_of_series_starts.get(&Series {
                                home_team: team,
                                away_team: other_team,
                                series_length: length,
                            }) {
                                let series_slot = vars.int_vars.get(idx).unwrap();
                                series_slot._eq(&Int::from_i64(context, slot as i64))
                            } else {
                                Bool::from_bool(context, false)
                            }
                        })
                    })
                    .flatten()
                    .collect::<Vec<Bool>>();

                // This is also silly: i have no better way to convert a
                // vector of objects to an array of references to those
                // objects, though
                let plays_at_home_vars_arr: [&Bool; 90] = plays_at_home_vars
                    .iter()
                    .collect::<Vec<&Bool>>()
                    .try_into()
                    .unwrap();

                let plays_at_home = Bool::or(context, &plays_at_home_vars_arr);
                vars.bool_vars.push(plays_at_home);
                vars.plays_at_home
                    .insert((team, slot), vars.bool_vars.len() - 1);

                // Similar comments as above apply, but even more so
                let plays_away_vars = [
                    (Division::NLWest, Division::ALWest),
                    (Division::NLCentral, Division::ALCentral),
                    (Division::NLEast, Division::ALEast),
                ]
                .map(|(al_division, nl_division)| {
                    al_division
                        .teams()
                        .iter()
                        .chain(nl_division.teams().iter())
                        .map(|&other_team| {
                            [2, 3, 4].map(|length| {
                                if let Some(&idx) = vars.slots_of_series_starts.get(&Series {
                                    home_team: other_team,
                                    away_team: team,
                                    series_length: length,
                                }) {
                                    let series_slot = vars.int_vars.get(idx).unwrap();
                                    series_slot._eq(&Int::from_i64(context, slot as i64))
                                } else {
                                    Bool::from_bool(context, false)
                                }
                            })
                        })
                        .flatten()
                        .collect::<Vec<Bool>>()
                });

                let away_west_vars_arr: [&Bool; 30] = plays_away_vars[0]
                    .iter()
                    .collect::<Vec<&Bool>>()
                    .try_into()
                    .unwrap();

                let away_central_vars_arr: [&Bool; 30] = plays_away_vars[1]
                    .iter()
                    .collect::<Vec<&Bool>>()
                    .try_into()
                    .unwrap();

                let away_east_vars_arr: [&Bool; 30] = plays_away_vars[2]
                    .iter()
                    .collect::<Vec<&Bool>>()
                    .try_into()
                    .unwrap();

                let plays_away_west = Bool::or(context, &away_west_vars_arr);
                let plays_away_central = Bool::or(context, &away_central_vars_arr);
                let plays_away_east = Bool::or(context, &away_east_vars_arr);

                let plays_away = Bool::or(
                    context,
                    &[&plays_away_west, &plays_away_central, &plays_away_east],
                );

                vars.bool_vars.push(plays_away_west);
                vars.plays_away_west
                    .insert((team, slot), vars.bool_vars.len() - 1);

                vars.bool_vars.push(plays_away_central);
                vars.plays_away_central
                    .insert((team, slot), vars.bool_vars.len() - 1);

                vars.bool_vars.push(plays_away_east);
                vars.plays_away_east
                    .insert((team, slot), vars.bool_vars.len() - 1);

                vars.bool_vars.push(plays_away);
                vars.plays_away
                    .insert((team, slot), vars.bool_vars.len() - 1);
            }
        }

        vars
    }
}
