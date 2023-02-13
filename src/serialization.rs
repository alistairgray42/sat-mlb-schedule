// so this is really not something that should exist, but for whatever reason z3
// doesn't compile if serde is in the Cargo.toml? sounds like a skill issue tbh
// but also not really that complicated to manually patch

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

pub fn serialization_slots(model: &Model, series: &Vec<Series>, vars: &SATVariables) -> Vec<u8> {
    series
        .iter()
        .map(|series| {
            let &var_idx = vars.slots_of_series_starts.get(&series).unwrap();
            let var = vars.int_vars.get(var_idx).unwrap();

            model
                .eval(var, false)
                .unwrap()
                .as_i64()
                .unwrap()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<u8>>()
}

pub fn serialize(file: &mut File, first_half_slots: Vec<u8>, second_half_slots: Vec<u8>) {
    let mut slots = first_half_slots.clone();
    slots.extend(second_half_slots);

    for slot in slots {
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
