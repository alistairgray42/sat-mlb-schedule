pub type SeriesSlot = i32;

// Teams play 27 weekend series and 25 weekday series during a season, starting
// on opening weekend and ending on closing weekend. The valid range for these
// slots is 0 through 52 (which is 53 total slots) because the all-star break is
// counted as an always-empty series slot for simplicity.

pub const FIRST_SERIES_SLOT: SeriesSlot = 0;
pub const ALL_STAR_SERIES_SLOT: SeriesSlot = 29; // based on observation of previous seasons
pub const LAST_SERIES_SLOT: SeriesSlot = 52;

pub const NUM_SLOTS: i32 = 53;

pub fn is_weekend_series(slot: &SeriesSlot) -> bool {
    slot % 2 == 0
}

pub fn is_midweek_series(slot: &SeriesSlot) -> bool {
    slot % 2 == 1
}
