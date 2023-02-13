use super::{
    division::Division::{self, *},
    team::Team,
};
use lazy_static::lazy_static;

pub enum League {
    American,
    National,
}

impl League {
    pub fn divisions(&self) -> Vec<Division> {
        use crate::definitions::league::League::*;
        match self {
            American => vec![ALWest, ALCentral, ALEast],
            National => vec![NLWest, NLCentral, NLEast],
        }
    }

    pub fn teams(&self) -> Vec<Team> {
        use crate::definitions::team::Team::*;
        match self {
            League::American => vec![
                Angels, Astros, Athletics, Mariners, Rangers, // AL West
                Guardians, Royals, Tigers, Twins, WhiteSox, // AL Central
                BlueJays, Orioles, Rays, RedSox, Yankees, // AL East
            ],
            League::National => vec![
                Diamondbacks,
                Dodgers,
                Giants,
                Padres,
                Rockies, // NL West
                Brewers,
                Cardinals,
                Cubs,
                Pirates,
                Reds, // NL Central
                Braves,
                Marlins,
                Mets,
                Nationals,
                Phillies, // NL East
            ],
        }
    }
}

lazy_static! {
    pub static ref LEAGUES: [League; 2] = [League::American, League::National];
}
