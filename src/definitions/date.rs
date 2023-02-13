// 1-indexed!
pub type Date = u32;

pub const FIRST_DATE: Date = 1; // let's go, don't wait, this night's almost over
pub const LAST_DATE: Date = 186;

pub const NUM_DATES: usize = (LAST_DATE - FIRST_DATE + 1) as usize;
pub const NUM_GAMES_IN_SEASON: usize = 162;
pub const NUM_REST_DAYS: usize = NUM_DATES - NUM_GAMES_IN_SEASON;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Month {
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
}

// this entire file is pretty embarassingly silly tbh

fn num_days_in_month(month: Month) -> u32 {
    use crate::definitions::date::Month::*;
    match month {
        March => 1, // yes
        April => 30,
        May => 31,
        June => 30,
        July => 31,
        August => 31,
        September => 30,
        October => 31,
    }
}

fn days_before_month(month: Month) -> u32 {
    use crate::definitions::date::Month::*;
    match month {
        March => 0,
        April => 1,
        May => 31,
        June => 62,
        July => 92,
        August => 123,
        September => 154,
        October => 184,
    }
}

// Operates assuming the original (before lockout rescheduling) dates of the
// first and last dates of the 2022 season:
// First day of the season is March 31, last is October 2
// Those are respectively Day 1 and Day 186 of the season
pub fn date_from_month_and_day(month: Month, day_of_month: u32) -> Result<Date, ()> {
    use crate::definitions::date::Month::*;
    match month {
        March => {
            if day_of_month == 31 {
                Ok(1)
            } else {
                Err(())
            }
        }
        October => {
            if day_of_month <= 2 {
                Ok(days_before_month(October) + day_of_month)
            } else {
                Err(())
            }
        }
        _ => Ok(days_before_month(month) + day_of_month),
    }
}

pub fn month_and_day_from_date(date: Date) -> Result<(Month, u32), ()> {
    use crate::definitions::date::Month::*;

    if date < FIRST_DATE {
        println!("wrong: {}", date);
        return Err(());
    } else if date > LAST_DATE {
        println!("wrong: {}", date);
        return Err(());
    } else if date == 1 {
        return Ok((March, 31));
    }

    for month in [April, May, June, July, August, September, October] {
        if date <= days_before_month(month) + num_days_in_month(month) {
            let day_of_month = date - days_before_month(month);
            return Ok((month, day_of_month));
        }
    }

    println!("wrong: {}", date);
    Err(())
}

#[test]
fn test_dates() {
    for original_date in FIRST_DATE..=LAST_DATE {
        print!("original date: {}; ", original_date);
        let (month, day_of_month) = month_and_day_from_date(original_date).unwrap();
        println!("month: {:?}, day: {}; ", month, day_of_month);
        let reconstructed_date = date_from_month_and_day(month, day_of_month).unwrap();
        assert_eq!(original_date, reconstructed_date);
    }
}
