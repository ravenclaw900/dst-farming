use crate::plant::{CropRatios, Plant, Seasons};

pub const fn get_combos(
    season: &Seasons,
    ratio: &CropRatios,
) -> Option<&'static [&'static [Plant]]> {
    match ratio {
        CropRatios::OneOne => one_one_lookup(season),
        CropRatios::OneOneOne => Some(one_one_one_lookup(season)),
        CropRatios::TwoOne => two_one_lookup(season),
        CropRatios::TwoOneOne => two_one_one_lookup(season),
    }
}

const fn one_one_lookup(season: &Seasons) -> Option<&'static [&'static [Plant]]> {
    match season {
        Seasons::Summer | Seasons::Winter => None,
        Seasons::Autumn => Some(&[
            &[Plant::TOMA_ROOT, Plant::EGGPLANT],
            &[Plant::TOMA_ROOT, Plant::POTATO],
        ]),
        Seasons::Spring => Some(&[
            &[Plant::CARROT, Plant::WATERMELON],
            &[Plant::POTATO, Plant::TOMA_ROOT],
            &[Plant::TOMA_ROOT, Plant::EGGPLANT],
        ]),
    }
}

const fn one_one_one_lookup(season: &Seasons) -> &'static [&'static [Plant]] {
    match season {
        Seasons::Summer => &[
            &[Plant::TOMA_ROOT, Plant::TOMA_ROOT, Plant::DRAGON_FRUIT],
            &[Plant::TOMA_ROOT, Plant::TOMA_ROOT, Plant::PEPPER],
            &[Plant::WATERMELON, Plant::WATERMELON, Plant::ONION],
            &[Plant::WATERMELON, Plant::WATERMELON, Plant::POMEGRANATE],
            &[Plant::DRAGON_FRUIT, Plant::GARLIC, Plant::ONION],
            &[Plant::DRAGON_FRUIT, Plant::GARLIC, Plant::POMEGRANATE],
            &[Plant::GARLIC, Plant::ONION, Plant::PEPPER],
            &[Plant::GARLIC, Plant::PEPPER, Plant::POMEGRANATE],
        ],
        Seasons::Winter => &[
            &[Plant::CARROT, Plant::POTATO, Plant::ASPARAGUS],
            &[Plant::POTATO, Plant::ASPARAGUS, Plant::PUMPKIN],
        ],
        Seasons::Autumn => &[
            &[Plant::CARROT, Plant::CORN, Plant::POTATO],
            &[Plant::CARROT, Plant::CORN, Plant::EGGPLANT],
            &[Plant::CORN, Plant::POTATO, Plant::PUMPKIN],
            &[Plant::CORN, Plant::EGGPLANT, Plant::PUMPKIN],
            &[Plant::TOMA_ROOT, Plant::TOMA_ROOT, Plant::PEPPER],
            &[Plant::GARLIC, Plant::ONION, Plant::PEPPER],
        ],
        Seasons::Spring => &[
            &[Plant::CARROT, Plant::CORN, Plant::POTATO],
            &[Plant::CARROT, Plant::CORN, Plant::EGGPLANT],
            &[Plant::CARROT, Plant::POTATO, Plant::ASPARAGUS],
            &[Plant::CARROT, Plant::ASPARAGUS, Plant::EGGPLANT],
            &[Plant::TOMA_ROOT, Plant::TOMA_ROOT, Plant::DRAGON_FRUIT],
            &[Plant::WATERMELON, Plant::WATERMELON, Plant::ONION],
            &[Plant::WATERMELON, Plant::WATERMELON, Plant::POMEGRANATE],
            &[Plant::DRAGON_FRUIT, Plant::DURIAN, Plant::ONION],
            &[Plant::DRAGON_FRUIT, Plant::DURIAN, Plant::POMEGRANATE],
            &[Plant::DRAGON_FRUIT, Plant::GARLIC, Plant::ONION],
            &[Plant::DRAGON_FRUIT, Plant::GARLIC, Plant::POMEGRANATE],
        ],
    }
}

const fn two_one_lookup(season: &Seasons) -> Option<&'static [&'static [Plant]]> {
    match season {
        Seasons::Summer => Some(&[
            &[Plant::TOMA_ROOT, Plant::TOMA_ROOT, Plant::DRAGON_FRUIT],
            &[Plant::TOMA_ROOT, Plant::TOMA_ROOT, Plant::PEPPER],
            &[Plant::WATERMELON, Plant::WATERMELON, Plant::ONION],
            &[Plant::WATERMELON, Plant::WATERMELON, Plant::POMEGRANATE],
        ]),
        Seasons::Winter => None,
        Seasons::Autumn => Some(&[&[Plant::TOMA_ROOT, Plant::TOMA_ROOT, Plant::PEPPER]]),
        Seasons::Spring => Some(&[
            &[Plant::TOMA_ROOT, Plant::TOMA_ROOT, Plant::DRAGON_FRUIT],
            &[Plant::WATERMELON, Plant::WATERMELON, Plant::ONION],
            &[Plant::WATERMELON, Plant::WATERMELON, Plant::POMEGRANATE],
        ]),
    }
}

#[allow(clippy::too_many_lines)]
const fn two_one_one_lookup(season: &Seasons) -> Option<&'static [&'static [Plant]]> {
    match season {
        Seasons::Summer => Some(&[
            &[Plant::CORN, Plant::CORN, Plant::DRAGON_FRUIT, Plant::ONION],
            &[
                Plant::CORN,
                Plant::CORN,
                Plant::DRAGON_FRUIT,
                Plant::POMEGRANATE,
            ],
            &[Plant::CORN, Plant::CORN, Plant::ONION, Plant::PEPPER],
            &[Plant::CORN, Plant::CORN, Plant::PEPPER, Plant::POMEGRANATE],
        ]),
        Seasons::Winter => None,
        Seasons::Autumn => Some(&[
            &[Plant::CARROT, Plant::CARROT, Plant::GARLIC, Plant::PEPPER],
            &[Plant::CORN, Plant::CORN, Plant::ONION, Plant::PEPPER],
            &[
                Plant::POTATO,
                Plant::POTATO,
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
            ],
            &[Plant::POTATO, Plant::POTATO, Plant::GARLIC, Plant::ONION],
            &[
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
                Plant::POTATO,
                Plant::POTATO,
            ],
            &[
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
                Plant::POTATO,
                Plant::EGGPLANT,
            ],
            &[
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
                Plant::EGGPLANT,
                Plant::EGGPLANT,
            ],
            &[
                Plant::EGGPLANT,
                Plant::EGGPLANT,
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
            ],
            &[
                Plant::EGGPLANT,
                Plant::EGGPLANT,
                Plant::GARLIC,
                Plant::ONION,
            ],
            &[Plant::PUMPKIN, Plant::PUMPKIN, Plant::GARLIC, Plant::PEPPER],
        ]),
        Seasons::Spring => Some(&[
            &[
                Plant::CARROT,
                Plant::CARROT,
                Plant::WATERMELON,
                Plant::WATERMELON,
            ],
            &[
                Plant::CARROT,
                Plant::CARROT,
                Plant::DRAGON_FRUIT,
                Plant::DURIAN,
            ],
            &[
                Plant::CARROT,
                Plant::CARROT,
                Plant::DRAGON_FRUIT,
                Plant::GARLIC,
            ],
            &[Plant::CORN, Plant::CORN, Plant::DRAGON_FRUIT, Plant::ONION],
            &[
                Plant::CORN,
                Plant::CORN,
                Plant::DRAGON_FRUIT,
                Plant::POMEGRANATE,
            ],
            &[
                Plant::POTATO,
                Plant::POTATO,
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
            ],
            &[Plant::POTATO, Plant::POTATO, Plant::DURIAN, Plant::ONION],
            &[
                Plant::POTATO,
                Plant::POTATO,
                Plant::DURIAN,
                Plant::POMEGRANATE,
            ],
            &[Plant::POTATO, Plant::POTATO, Plant::GARLIC, Plant::ONION],
            &[
                Plant::POTATO,
                Plant::POTATO,
                Plant::GARLIC,
                Plant::POMEGRANATE,
            ],
            &[
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
                Plant::POTATO,
                Plant::POTATO,
            ],
            &[
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
                Plant::POTATO,
                Plant::EGGPLANT,
            ],
            &[
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
                Plant::EGGPLANT,
                Plant::EGGPLANT,
            ],
            &[
                Plant::ASPARAGUS,
                Plant::ASPARAGUS,
                Plant::DRAGON_FRUIT,
                Plant::ONION,
            ],
            &[
                Plant::ASPARAGUS,
                Plant::ASPARAGUS,
                Plant::DRAGON_FRUIT,
                Plant::POMEGRANATE,
            ],
            &[
                Plant::EGGPLANT,
                Plant::EGGPLANT,
                Plant::TOMA_ROOT,
                Plant::TOMA_ROOT,
            ],
            &[
                Plant::EGGPLANT,
                Plant::EGGPLANT,
                Plant::DURIAN,
                Plant::ONION,
            ],
            &[
                Plant::EGGPLANT,
                Plant::EGGPLANT,
                Plant::DURIAN,
                Plant::POMEGRANATE,
            ],
            &[
                Plant::EGGPLANT,
                Plant::EGGPLANT,
                Plant::GARLIC,
                Plant::ONION,
            ],
            &[
                Plant::EGGPLANT,
                Plant::EGGPLANT,
                Plant::GARLIC,
                Plant::POMEGRANATE,
            ],
            &[
                Plant::WATERMELON,
                Plant::WATERMELON,
                Plant::CARROT,
                Plant::CARROT,
            ],
        ]),
    }
}
