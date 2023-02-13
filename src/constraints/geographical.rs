pub fn assert_geographical_constraints<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
) {
    // this is a pretty loose constraint
    assert_roadtrips_follow_divisional_geography(context, solver, vars, series);
}

// Divide between western, central, and eastern teams based on divisional
// alignment - for all teams, any given roadtrip will only include series
// against teams of one geographical area (e.g. you can have a series against
// both NL and AL Central teams on the same road trip, but not teams from the
// Wests or Easts)

pub fn assert_roadtrips_follow_divisional_geography<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
) {
}
