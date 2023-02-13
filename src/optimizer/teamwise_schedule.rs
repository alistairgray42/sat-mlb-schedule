use std::collections::HashMap;

use crate::definitions::{series::Series, series_slot::SeriesSlot, team::Team};

pub type TeamWiseSchedule = HashMap<(Team, SeriesSlot), Series>;

pub fn to_teamwise_schedule(schedule: &HashMap<Series, SeriesSlot>) -> TeamWiseSchedule {
    let mut res = HashMap::new();

    for (&series, &slot) in schedule.iter() {
        res.insert((series.home_team, slot), series);
        res.insert((series.away_team, slot), series);
    }

    res
}
