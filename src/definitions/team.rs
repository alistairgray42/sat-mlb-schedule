use lazy_static::lazy_static;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum Team {
    // NL West
    Diamondbacks,
    Dodgers,
    Giants,
    Padres,
    Rockies,

    // NL Central
    Brewers,
    Cardinals,
    Cubs,
    Pirates,
    Reds,

    // NL East
    Braves,
    Marlins,
    Mets,
    Nationals,
    Phillies,

    // AL West
    Angels,
    Astros,
    Athletics,
    Mariners,
    Rangers,

    // AL Central
    Guardians,
    Royals,
    Tigers,
    Twins,
    WhiteSox,

    // AL East
    BlueJays,
    Orioles,
    Rays,
    RedSox,
    Yankees,
}

impl Team {
    pub fn rival(&self) -> Team {
        use crate::definitions::team::Team::*;
        match self {
            Diamondbacks => Astros, // registering a complaint that this makes no sense
            Dodgers => Angels,
            Giants => Athletics,
            Padres => Mariners,
            Rockies => Rangers, // registering a complaint that this makes no sense

            Brewers => Twins,
            Cardinals => Royals,
            Cubs => WhiteSox,
            Pirates => Tigers,
            Reds => Guardians,

            Braves => RedSox, // registering a complaint that this makes no sense
            Marlins => Rays,
            Mets => Yankees,
            Nationals => Orioles,
            Phillies => BlueJays, // registering a complaint that this makes no sense

            Angels => Dodgers,
            Astros => Diamondbacks, // registering a complaint that this makes no sense
            Athletics => Giants,
            Mariners => Padres,
            Rangers => Rockies, // registering a complaint that this makes no sense

            Guardians => Reds,
            Royals => Cardinals,
            Tigers => Pirates,
            Twins => Brewers,
            WhiteSox => Cubs,

            BlueJays => Phillies, // registering a complaint that this makes no sense
            Orioles => Nationals,
            Rays => Marlins,
            RedSox => Braves, // registering a complaint that this makes no sense
            Yankees => Mets,
        }
    }
}

lazy_static! {
    pub static ref TEAMS: [Team; 30] = [
        Team::Diamondbacks,
        Team::Dodgers,
        Team::Giants,
        Team::Padres,
        Team::Rockies,
        Team::Brewers,
        Team::Cardinals,
        Team::Cubs,
        Team::Pirates,
        Team::Reds,
        Team::Braves,
        Team::Marlins,
        Team::Mets,
        Team::Nationals,
        Team::Phillies,
        Team::Angels,
        Team::Astros,
        Team::Athletics,
        Team::Mariners,
        Team::Rangers,
        Team::Guardians,
        Team::Royals,
        Team::Tigers,
        Team::Twins,
        Team::WhiteSox,
        Team::BlueJays,
        Team::Orioles,
        Team::Rays,
        Team::RedSox,
        Team::Yankees,
    ];
}
