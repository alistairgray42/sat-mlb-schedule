use std::{collections::HashMap, fs::File, io::Write};

use definitions::series::Series;
use serialization::deserialize;
use z3::{Config, Context, Solver};

use crate::{
    constraints::assert_all_constraints,
    definitions::{team::TEAMS, variables::SATVariables},
    pretty_print_schedule::pretty_print_schedule,
    serialization::serialize,
    series::generate_all_series,
};

mod constraints;
mod definitions;
mod optimizer;
mod pretty_print_schedule;
mod serialization;
mod series;

fn generate_and_serialize() {
    let (first_half_series, second_half_series) = generate_all_series();

    let mut config = Config::new();
    config.set_model_generation(true);
    let context = Context::new(&config);

    let first_half_vars = SATVariables::new(&context, &first_half_series, true);
    let mut solver = Solver::new(&context);

    println!("Generating first half constraints");
    assert_all_constraints(
        &context,
        &mut solver,
        &first_half_vars,
        &first_half_series,
        &TEAMS.to_vec(),
        true,
    );
    println!("Generating first half schedule");
    assert!(matches!(solver.check(), z3::SatResult::Sat));
    let first_half_model = solver.get_model().unwrap();

    let second_half_vars = SATVariables::new(&context, &second_half_series, false);
    let mut solver = Solver::new(&context);

    println!("Generating second half constraints");
    assert_all_constraints(
        &context,
        &mut solver,
        &second_half_vars,
        &second_half_series,
        &TEAMS.to_vec(),
        false,
    );
    println!("Generating second half schedule");
    assert!(matches!(solver.check(), z3::SatResult::Sat));
    let second_half_model = solver.get_model().unwrap();

    let mut all_series = first_half_series.clone();
    all_series.extend(second_half_series);
    println!("length: {}", all_series.len());

    println!("Done! Writing file output");

    let mut file = File::create("schedule_serialized").expect("Couldn't create file!");

    serialize(
        &mut file,
        first_half_model,
        second_half_model,
        all_series,
        first_half_vars,
        second_half_vars,
    );

    println!("All done!");
}

fn deserialize_schedule() -> HashMap<Series, i32> {
    let (first_half_series, second_half_series) = generate_all_series();
    let mut all_series = first_half_series.clone();
    all_series.extend(second_half_series);

    let mut serialized_file = File::open("schedule_serialized").expect("Couldn't open file!");

    deserialize(&mut serialized_file, all_series)
}

fn pretty_print(schedule: HashMap<Series, i32>) {
    let lines = pretty_print_schedule(schedule);

    let mut schedule_file = File::create("schedule_pretty").expect("Couldn't open file!");
    for line in lines {
        schedule_file
            .write_all(line.as_bytes())
            .expect("Couldn't write to file!");
    }
}

fn main() {
    // generate_and_serialize();
    let schedule = deserialize_schedule();
    // pretty_print(schedule);
}
