use std::collections::HashMap;

use crate::definitions::{
    series::Series,
    series_slot::{SeriesSlot, ALL_STAR_SERIES_SLOT},
    team::TEAMS,
};

pub fn pretty_print_schedule(series_to_slot: HashMap<Series, i32>) -> Vec<String> {
    let mut lines = vec![];

    let series: Vec<&Series> = series_to_slot.keys().collect();

    for &team in TEAMS.iter() {
        let applicable_series: Vec<&Series> = series
            .iter()
            .filter(|&s| s.home_team == team || s.away_team == team)
            .map(|&s| s)
            .collect();

        let mut series_with_slots: Vec<(SeriesSlot, &Series)> = applicable_series
            .iter()
            .map(|&series| (series_to_slot.get(series).unwrap().to_owned(), series))
            .collect();

        series_with_slots.sort_by(|a, b| a.0.cmp(&b.0));

        lines.push(format!("\n{:?}\n------------\n", team));

        for (slot, series) in series_with_slots {
            if series.home_team == team {
                lines.push(format!(
                    "{}: vs {:?} ({})\n",
                    slot, series.away_team, series.series_length
                ));
            } else if series.away_team == team {
                lines.push(format!(
                    "{}: at {:?} ({})\n",
                    slot, series.home_team, series.series_length
                ));
            }

            if slot == ALL_STAR_SERIES_SLOT - 1 {
                lines.push(format!("{}: All-Star Break\n", ALL_STAR_SERIES_SLOT));
            }
        }
    }

    lines
}
