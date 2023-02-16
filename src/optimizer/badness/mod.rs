use self::{
    divisional_opponents_too_close::divisional_opponents_too_close_badness,
    homestand_roadtrip_length::homestand_roadtrip_length_badness,
    identical_opponents_too_close::identical_opponents_too_close_badness,
    roadtrip_distance_traveled::roadtrip_distance_traveled_badness,
};

use super::teamwise_schedule::TeamWiseSchedule;

mod divisional_opponents_too_close;
mod homestand_roadtrip_length;
mod identical_opponents_too_close;
mod roadtrip_distance_traveled;

pub fn badness(schedule: TeamWiseSchedule) -> f32 {
    return divisional_opponents_too_close_badness(&schedule)
        + homestand_roadtrip_length_badness(&schedule)
        + identical_opponents_too_close_badness(&schedule)
        + roadtrip_distance_traveled_badness(&schedule);
}
