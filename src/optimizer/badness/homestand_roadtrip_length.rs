use std::collections::HashMap;

use crate::{
    definitions::{
        series::Series,
        series_slot::{SeriesSlot, ALL_STAR_SERIES_SLOT, FIRST_SERIES_SLOT, LAST_SERIES_SLOT},
        team::Team,
    },
    optimizer::ScheduleArray,
};

pub fn homestand_roadtrip_length_badness(schedule: &ScheduleArray) -> f32 {
    let mut m: HashMap<Team, HashMap<SeriesSlot, Series>> = HashMap::new();
    for (slot, series_in_slot) in schedule.iter().enumerate() {
        let final_slot = if (slot as i32) < ALL_STAR_SERIES_SLOT {
            slot
        } else {
            slot + 1
        } as i32;

        for &series in series_in_slot.iter() {
            if !m.contains_key(&series.home_team) {
                m.insert(series.home_team, HashMap::new());
            }
            if !m.contains_key(&series.away_team) {
                m.insert(series.away_team, HashMap::new());
            }

            m.get_mut(&series.home_team)
                .unwrap()
                .insert(final_slot, series);
            m.get_mut(&series.away_team)
                .unwrap()
                .insert(final_slot, series);
        }
    }

    m.iter()
        .map(|(&team, schedule)| homestand_roadtrip_length_badness_team(team, schedule))
        .reduce(|acc, elem| acc + elem)
        .unwrap()
}

fn homestand_roadtrip_length_badness_team(
    team: Team,
    schedule: &HashMap<SeriesSlot, Series>,
) -> f32 {
    let mut total = 0.0;
    // special case for the first of the half
    let mut current_len = 1;
    let mut is_currently_home = schedule.get(&FIRST_SERIES_SLOT).unwrap().home_team == team;

    for slot in FIRST_SERIES_SLOT + 1..ALL_STAR_SERIES_SLOT {
        let this_slot_is_home = schedule.get(&slot).unwrap().home_team == team;

        if is_currently_home == this_slot_is_home {
            current_len += 1;
        } else {
            // one-series homestands or roadtrips are acceptable as first or
            // last homestands/roadtrips of the half, but otherwise isn't,
            // and four or more is never permissible

            if current_len == 1 && slot > FIRST_SERIES_SLOT + 1 {
                total += 100.0;
            } else if current_len > 3 {
                total += 100.0 * (current_len - 3) as f32;
            }

            current_len = 1;
            is_currently_home = this_slot_is_home;
        }

        // in case the last homestand/roadtrip is too long
        if current_len > 3 {
            total += 100.0 * (current_len - 3) as f32;
        }
    }

    // Now do the same again for the second half
    current_len = 1;
    is_currently_home = schedule.get(&(ALL_STAR_SERIES_SLOT + 1)).unwrap().home_team == team;

    for slot in ALL_STAR_SERIES_SLOT + 2..=LAST_SERIES_SLOT {
        let this_slot_is_home = schedule.get(&slot).unwrap().home_team == team;

        if is_currently_home == this_slot_is_home {
            current_len += 1;
        } else {
            // one-series homestands or roadtrips are acceptable as first or
            // last homestands/roadtrips of the half, but otherwise isn't,
            // and four or more is never permissible

            if current_len == 1 && slot > ALL_STAR_SERIES_SLOT + 2 {
                total += 100.0;
            } else if current_len > 3 {
                total += 100.0 * (current_len - 3) as f32;
            }

            current_len = 1;
            is_currently_home = this_slot_is_home;
        }

        // in case the last homestand/roadtrip is too long
        if current_len > 3 {
            total += 100.0 * (current_len - 3) as f32;
        }
    }

    total
}
