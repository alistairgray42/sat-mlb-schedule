// use serde::{Deserialize, Serialize};

use super::team::Team;
use std::fmt::Debug;

// 52 series per team, times 30 teams, divided by 2 for overcounting
pub const TOTAL_NUMBER_OF_SERIES: i32 = 780;

// #[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Series {
    pub home_team: Team,
    pub away_team: Team,
    pub series_length: u32,
}

impl Debug for Series {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Series")
            .field("home_team", &self.home_team)
            .field("away_team", &self.away_team)
            .field("series_length", &self.series_length)
            .finish()
    }
}
