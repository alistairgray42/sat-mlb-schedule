// so this is really not something that should exist, but for whatever reason z3
// doesn't compile if z3 is in the Cargo.toml? sounds like a skill issue tbh but
// also not really that complicated to manually patch

// dubious assumption made for the sake of simplicity: series are in the same
// order when serialized and deserialized. this allows us to
// serialize/deserialize using just the slot numbers for each series in order

use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

use z3::Model;

use crate::definitions::{
    series::{Series, TOTAL_NUMBER_OF_SERIES},
    variables::SATVariables,
};

pub fn serialize(
    file: &mut File,
    first_half_model: Model,
    second_half_model: Model,
    series: Vec<Series>,
    first_half_vars: SATVariables,
    second_half_vars: SATVariables,
) {
    for series in series {
        let slot: u8 = if first_half_vars.slots_of_series_starts.contains_key(&series) {
            let &var_idx = first_half_vars.slots_of_series_starts.get(&series).unwrap();
            let var = first_half_vars.int_vars.get(var_idx).unwrap();

            first_half_model
                .eval(var, false)
                .unwrap()
                .as_i64()
                .unwrap()
                .try_into()
                .unwrap()
        } else {
            let &var_idx = second_half_vars
                .slots_of_series_starts
                .get(&series)
                .unwrap();
            let var = second_half_vars.int_vars.get(var_idx).unwrap();

            second_half_model
                .eval(var, false)
                .unwrap()
                .as_i64()
                .unwrap()
                .try_into()
                .unwrap()
        };
        file.write(&[slot]).expect("Couldn't write to file!");
    }
}

pub fn deserialize(file: &mut File, series: Vec<Series>) -> HashMap<Series, i32> {
    let mut buf = [0; TOTAL_NUMBER_OF_SERIES as usize];
    file.read(&mut buf).expect("Couldn't read from file!");

    let mut res = HashMap::new();

    for index in 0..TOTAL_NUMBER_OF_SERIES {
        let series = series[index as usize];
        let slot = buf[index as usize] as i32;

        res.insert(series, slot);
    }

    res
}
