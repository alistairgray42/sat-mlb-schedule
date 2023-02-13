use crate::definitions::{
    division::{Division, DIVISIONS},
    league::{League, LEAGUES},
    series::Series,
    team::{Team, TEAMS},
};

pub fn generate_divisional_series() -> (Vec<Series>, Vec<Series>) {
    let mut first_half = vec![];
    let mut second_half = vec![];

    for division in DIVISIONS.iter() {
        for i in 0..division.teams().len() {
            for j in i + 1..division.teams().len() {
                let team_1 = division.teams()[i];
                let team_2 = division.teams()[j];
                first_half.append(&mut vec![
                    Series {
                        home_team: team_1,
                        away_team: team_2,
                        series_length: 3,
                    },
                    Series {
                        home_team: team_2,
                        away_team: team_1,
                        series_length: 4,
                    },
                ]);
                second_half.append(&mut vec![
                    Series {
                        home_team: team_1,
                        away_team: team_2,
                        series_length: 4,
                    },
                    Series {
                        home_team: team_2,
                        away_team: team_1,
                        series_length: 3,
                    },
                ]);
            }
        }
    }

    (first_half, second_half)
}

fn generate_intra_league_series() -> (Vec<Series>, Vec<Series>) {
    // put "half" in quotes: these will respectively contain 12 and 8 series per
    // team. each team plays each team in the other divisions for one series in
    // the first half, and one additional series against a team in each of the
    // other two divisions; all other series are in the second half
    let mut first_half = vec![];
    let mut second_half = vec![];

    // Intra-league games: each team plays 6 games against each of the 10 teams
    // in its league but not in its division. This is a 3-game series home and a
    // 3-game series away
    for league in LEAGUES.iter() {
        for (division_1_idx, division_2_idx) in [(0, 1), (0, 2), (1, 2)] {
            let division_1 = league.divisions()[division_1_idx];
            let division_2 = league.divisions()[division_2_idx];

            for (team_1_idx, &team_1) in division_1.teams().iter().enumerate() {
                for (team_2_idx, &team_2) in division_2.teams().iter().enumerate() {
                    // Using this alone would be the even split needed to have 10
                    // interleague games in the first and in the second halves
                    let even_split_team_1_at_home =
                        (team_1_idx + team_2_idx + division_1_idx + division_2_idx) % 2 == 0;

                    let first_half_series = if even_split_team_1_at_home {
                        Series {
                            home_team: team_1,
                            away_team: team_2,
                            series_length: 3,
                        }
                    } else {
                        Series {
                            home_team: team_2,
                            away_team: team_1,
                            series_length: 3,
                        }
                    };

                    let maybe_second_half_series = if even_split_team_1_at_home {
                        Series {
                            home_team: team_2,
                            away_team: team_1,
                            series_length: 3,
                        }
                    } else {
                        Series {
                            home_team: team_1,
                            away_team: team_2,
                            series_length: 3,
                        }
                    };

                    first_half.push(first_half_series);

                    // Go from 10/10 to 12/8
                    if team_1_idx == team_2_idx {
                        first_half.push(maybe_second_half_series);
                    } else {
                        second_half.push(maybe_second_half_series);
                    }
                }
            }
        }
    }

    (first_half, second_half)
}

fn generate_rivalry_series() -> (Vec<Series>, Vec<Series>) {
    let mut first_half = vec![];
    let mut second_half = vec![];

    // Inter-league "rivalry" games: 4 games, home-and-home series of two
    for (index, &al_team) in League::American.teams().iter().enumerate() {
        let nl_team = al_team.rival();
        first_half.append(&mut vec![Series {
            home_team: if index % 2 == 0 { nl_team } else { al_team },
            away_team: if index % 2 == 0 { al_team } else { nl_team },
            series_length: 2,
        }]);
        second_half.append(&mut vec![Series {
            home_team: if index % 2 == 0 { al_team } else { nl_team },
            away_team: if index % 2 == 0 { nl_team } else { al_team },
            series_length: 2,
        }]);
    }

    (first_half, second_half)
}

fn generate_non_rivalry_inter_league_series() -> (Vec<Series>, Vec<Series>) {
    let mut first_half = vec![];
    let mut second_half = vec![];
    // Other inter-league games: each team plays 3 games against the 14 other teams in the other league
    let sorted_al_teams = League::American.teams();
    let sorted_nl_teams: Vec<Team> = sorted_al_teams.iter().map(|t| t.rival()).collect();

    for (al_team_index, &al_team) in sorted_al_teams.iter().enumerate() {
        for (nl_team_index, &nl_team) in sorted_nl_teams.iter().enumerate() {
            if nl_team == al_team.rival() {
                // equivalent to indices being equal
                continue;
            }

            // since there's only a single 3-game series between two
            // non-rival teams from different leagues, it will
            // alternate which of the two will be the home team (and
            // the information about which will be the home team in
            // 2023 is not available / probably doesn't exist) - as
            // such, doing this balances it out so that every team has 7 of
            // these series at home and 7 on the road

            let al_is_home = (al_team_index + nl_team_index) % 2 == 1;
            let series = Series {
                home_team: if al_is_home { al_team } else { nl_team },
                away_team: if al_is_home { nl_team } else { al_team },
                series_length: 3,
            };

            // we want eight of the fourteen interleague series that each
            // team plays to be in the first "half"
            // this math ensures that 8 of each 14 are thus chosen;
            // source: trust me

            let series_in_first_half = (al_team_index > nl_team_index
                && al_team_index <= nl_team_index + 8)
                || (nl_team_index >= 7 && al_team_index <= nl_team_index - 7);

            if series_in_first_half {
                first_half.push(series);
            } else {
                second_half.push(series);
            }
        }
    }

    (first_half, second_half)
}

// Generate lists of the first-half and second-half series
pub fn generate_all_series() -> (Vec<Series>, Vec<Series>) {
    let mut first_half = vec![];
    let mut second_half = vec![];

    for (mut first, mut second) in vec![
        generate_divisional_series(),
        generate_intra_league_series(),
        generate_rivalry_series(),
        generate_non_rivalry_inter_league_series(),
    ] {
        first_half.append(&mut first);
        second_half.append(&mut second);
    }

    (first_half, second_half)
}
