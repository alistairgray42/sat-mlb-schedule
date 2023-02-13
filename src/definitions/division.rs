use super::team::Team::{self, *};
use lazy_static::lazy_static;

#[derive(Clone, Copy)]
pub enum Division {
    NLWest,
    NLCentral,
    NLEast,
    ALWest,
    ALCentral,
    ALEast,
}

impl Division {
    pub fn teams(&self) -> Vec<Team> {
        match self {
            Division::NLWest => vec![Diamondbacks, Dodgers, Giants, Padres, Rockies],
            Division::NLCentral => vec![Brewers, Cardinals, Cubs, Pirates, Reds],
            Division::NLEast => vec![Braves, Marlins, Mets, Nationals, Phillies],
            Division::ALWest => vec![Angels, Astros, Athletics, Mariners, Rangers],
            Division::ALCentral => vec![Guardians, Royals, Tigers, Twins, WhiteSox],
            Division::ALEast => vec![BlueJays, Orioles, Rays, RedSox, Yankees],
        }
    }
}

lazy_static! {
    pub static ref DIVISIONS: [Division; 6] = [
        Division::NLWest,
        Division::NLCentral,
        Division::NLEast,
        Division::ALWest,
        Division::ALCentral,
        Division::ALEast
    ];
}
