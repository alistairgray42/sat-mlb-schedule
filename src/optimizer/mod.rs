use std::collections::HashMap;

use crate::definitions::series::Series;

use self::teamwise_schedule::TeamWiseSchedule;

pub mod badness;
mod teamwise_schedule;

pub fn optimize_homestand_roadtrip_length_badness_single_step(schedule: HashMap<Series, i32>) {}
