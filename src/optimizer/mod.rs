use std::collections::HashMap;

use crate::badness;

use crate::definitions::{
    series::Series,
    series_slot::{ALL_STAR_SERIES_SLOT, FIRST_SERIES_SLOT, LAST_SERIES_SLOT, NUM_SLOTS},
};

pub mod badness;
mod teamwise_schedule;

pub type ScheduleArray = [[Series; 15]; 52];

fn schedule_hashmap_to_array(schedule: &HashMap<Series, i32>) -> ScheduleArray {
    let mut slot_to_series = [[None; 15]; NUM_SLOTS as usize];
    let mut last_col_used = [0; NUM_SLOTS as usize];

    for (series, slot) in schedule.iter() {
        let idx = *slot as usize;
        slot_to_series[idx][last_col_used[idx]] = Some(*series);
        last_col_used[idx] += 1;
    }

    let first_half_sched = TryInto::<[[Series; 15]; 29]>::try_into(
        (FIRST_SERIES_SLOT..ALL_STAR_SERIES_SLOT)
            .map(|row| {
                TryInto::<[Series; 15]>::try_into(
                    (0..15)
                        .map(move |col| slot_to_series[row as usize][col].unwrap())
                        .collect::<Vec<Series>>(),
                )
                .unwrap()
            })
            .collect::<Vec<[Series; 15]>>(),
    )
    .unwrap();

    let second_half_sched = TryInto::<[[Series; 15]; 23]>::try_into(
        (ALL_STAR_SERIES_SLOT + 1..=LAST_SERIES_SLOT)
            .map(|row| {
                TryInto::<[Series; 15]>::try_into(
                    (0..15)
                        .map(move |col| slot_to_series[row as usize][col].unwrap())
                        .collect::<Vec<Series>>(),
                )
                .unwrap()
            })
            .collect::<Vec<[Series; 15]>>(),
    )
    .unwrap();

    first_half_sched
        .into_iter()
        .chain(second_half_sched.into_iter())
        .collect::<Vec<[Series; 15]>>()
        .try_into()
        .unwrap()
}

fn schedule_array_to_hashmap(schedule: ScheduleArray) -> HashMap<Series, i32> {
    let mut map = HashMap::<Series, i32>::new();

    for row in 0..52 {
        let row_num = if row < ALL_STAR_SERIES_SLOT {
            row
        } else {
            row + 1
        };
        for col in 0..15 {
            map.insert(schedule[row as usize][col], row_num);
        }
    }

    map
}

fn maybe_perturb_two_rows(
    schedule: ScheduleArray,
    badness_score: f32,
) -> Option<(ScheduleArray, f32, usize, usize)> {
    for row_1 in 0..51 {
        for row_2 in (row_1 + 1)..52 {
            let mut schedule_copy = schedule.clone();

            let tmp = schedule_copy[row_1];
            schedule_copy[row_1] = schedule_copy[row_2];
            schedule_copy[row_2] = tmp;

            let score = badness(&schedule_copy);

            if score < badness_score {
                return Some((schedule_copy, score, row_1, row_2));
            }
        }
    }
    return None;
}

fn maybe_perturb_three_rows(
    schedule: ScheduleArray,
    badness_score: f32,
) -> Option<(ScheduleArray, f32, usize, usize, usize)> {
    for row_1 in 0..50 {
        for row_2 in (row_1 + 1)..51 {
            for row_3 in (row_2 + 1)..52 {
                let mut schedule_copy = schedule.clone();

                let tmp = schedule_copy[row_1];
                schedule_copy[row_1] = schedule_copy[row_2];
                schedule_copy[row_2] = schedule_copy[row_3];
                schedule_copy[row_3] = tmp;

                let score = badness(&schedule_copy);

                if score < badness_score {
                    return Some((schedule_copy, score, row_1, row_2, row_3));
                }
            }
        }
    }
    return None;
}

pub fn try_some_perturbations(initial_schedule: &HashMap<Series, i32>) -> HashMap<Series, i32> {
    // Get the schedule into tabular form for easier manipulation
    let mut schedule = schedule_hashmap_to_array(initial_schedule);
    let mut badness_score = badness(&schedule);

    let mut iteration_num = 0;

    loop {
        iteration_num += 1;

        let two_row_result = maybe_perturb_two_rows(schedule, badness_score);
        if two_row_result.is_some() {
            let row_1;
            let row_2;
            let old_score = badness_score;

            ((schedule, badness_score, row_1, row_2)) = two_row_result.unwrap();
            println!(
                "[{iteration_num}]: replaced ({row_1}, {row_2}); {old_score} < {badness_score}"
            );
        }

        let three_row_result = maybe_perturb_three_rows(schedule, badness_score);
        if three_row_result.is_some() {
            let row_1;
            let row_2;
            let row_3;
            let old_score = badness_score;

            ((schedule, badness_score, row_1, row_2, row_3)) = three_row_result.unwrap();
            println!("[{iteration_num}]: cycled ({row_1} <- {row_2} <- {row_3}); {old_score} < {badness_score}");
        }

        if two_row_result.is_none() && three_row_result.is_none() {
            break;
        }
    }

    return schedule_array_to_hashmap(schedule);
}
