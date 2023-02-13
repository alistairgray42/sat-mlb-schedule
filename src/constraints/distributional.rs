use z3::{ast::Bool, Context, Solver};

use crate::definitions::{series::Series, variables::SATVariables};

pub fn assert_distributional_constraints<'a>(
    context: &'a Context,
    solver: &mut Solver,
    vars: &SATVariables,
    series: &Vec<Series>,
) {
}

// You always hear fans complain like "we don't play the dodgers at all in April or May!?"
// pub fn assert_divisional_opponents_play_early_and_late_enough<'a>)(){}
