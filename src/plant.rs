use ansi_term::Color;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
#[allow(clippy::struct_excessive_bools)]
struct Season {
    autumn: bool,
    winter: bool,
    spring: bool,
    summer: bool,
}

#[derive(Debug, PartialEq)]
pub struct Plant {
    pub name: &'static str,
    abbrev: &'static str,
    seasons: Season,
    pub formula: i8,
    pub compost: i8,
    pub manure: i8,
    pub color: Color,
}

pub enum Seasons {
    Autumn,
    Winter,
    Spring,
    Summer,
}

impl FromStr for Seasons {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "Autumn" => Self::Autumn,
            "Winter" => Self::Winter,
            "Spring" => Self::Spring,
            "Summer" => Self::Summer,
            _ => Err(anyhow::anyhow!("Couldn't parse season from {}", s))?,
        })
    }
}

#[allow(clippy::enum_variant_names)]
pub enum CropRatios {
    OneOneOne,
    OneOne,
    TwoOne,
    TwoOneOne,
}

impl FromStr for CropRatios {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "1:1:1" => Self::OneOneOne,
            "1:1" => Self::OneOne,
            "2:1" => Self::TwoOne,
            "2:1:1" => Self::TwoOneOne,
            _ => Err(anyhow::anyhow!("Couldn't parse crop ratio from {}", s))?,
        })
    }
}

impl CropRatios {
    pub const fn get_min_seeds_per_crop(&self) -> u32 {
        match self {
            Self::OneOne => 4,
            Self::OneOneOne => 12,
            Self::TwoOne => 6,
            Self::TwoOneOne => 8,
        }
    }

    pub const fn get_filled_size(&self, size: &FarmSize) -> u16 {
        self.get_filled_size_horizontal(size) * self.get_filled_size_vertical(size)
    }

    pub const fn get_filled_size_horizontal(&self, size: &FarmSize) -> u16 {
        match self {
            Self::OneOne => size.width,
            _ => size.width / 2,
        }
    }

    pub const fn get_filled_size_vertical(&self, size: &FarmSize) -> u16 {
        match self {
            Self::OneOne | Self::TwoOne => size.height,
            Self::OneOneOne | Self::TwoOneOne => size.height / 2,
        }
    }
}

pub struct FarmSize {
    pub width: u16,
    pub height: u16,
}

impl Plant {
    pub const fn in_season(&self, season: &Seasons) -> bool {
        match season {
            Seasons::Autumn => self.seasons.autumn,
            Seasons::Winter => self.seasons.winter,
            Seasons::Spring => self.seasons.spring,
            Seasons::Summer => self.seasons.summer,
        }
    }
    pub const CARROT: Self = Self {
        name: "Carrot",
        abbrev: "Ct",
        seasons: Season {
            autumn: true,
            winter: true,
            spring: true,
            summer: false,
        },
        formula: -4,
        compost: 2,
        manure: 2,
        color: Color::Fixed(208),
    };
    pub const CORN: Self = Self {
        name: "Corn",
        abbrev: "Cn",
        seasons: Season {
            autumn: true,
            winter: false,
            spring: true,
            summer: true,
        },
        formula: 2,
        compost: -4,
        manure: 2,
        color: Color::Fixed(11),
    };
    pub const POTATO: Self = Self {
        name: "Potato",
        abbrev: "Po",
        seasons: Season {
            autumn: true,
            winter: true,
            spring: true,
            summer: false,
        },
        formula: 2,
        compost: 2,
        manure: -4,
        color: Color::Fixed(3),
    };
    pub const TOMA_ROOT: Self = Self {
        name: "Toma Root",
        abbrev: "TR",
        seasons: Season {
            autumn: true,
            winter: false,
            spring: true,
            summer: true,
        },
        formula: -2,
        compost: -2,
        manure: 4,
        color: Color::Fixed(9),
    };
    pub const ASPARAGUS: Self = Self {
        name: "Asparagus",
        abbrev: "As",
        seasons: Season {
            autumn: false,
            winter: true,
            spring: true,
            summer: false,
        },
        formula: 2,
        compost: -4,
        manure: 2,
        color: Color::Fixed(2),
    };
    pub const EGGPLANT: Self = Self {
        name: "Eggplant",
        abbrev: "Eg",
        seasons: Season {
            autumn: true,
            winter: false,
            spring: true,
            summer: false,
        },
        formula: 2,
        compost: 2,
        manure: -4,
        color: Color::Fixed(5),
    };
    pub const PUMPKIN: Self = Self {
        name: "Pumpkin",
        abbrev: "Pk",
        seasons: Season {
            autumn: true,
            winter: true,
            spring: false,
            summer: false,
        },
        formula: -4,
        compost: 2,
        manure: 2,
        color: Color::Fixed(172),
    };
    pub const WATERMELON: Self = Self {
        name: "Watermelon",
        abbrev: "Wm",
        seasons: Season {
            autumn: false,
            winter: false,
            spring: true,
            summer: true,
        },
        formula: 4,
        compost: -2,
        manure: -2,
        color: Color::Fixed(10),
    };
    pub const DRAGON_FRUIT: Self = Self {
        name: "Dragon Fruit",
        abbrev: "DF",
        seasons: Season {
            autumn: false,
            winter: false,
            spring: true,
            summer: true,
        },
        formula: 4,
        compost: 4,
        manure: -8,
        color: Color::Fixed(13),
    };
    pub const DURIAN: Self = Self {
        name: "Durian",
        abbrev: "Du",
        seasons: Season {
            autumn: false,
            winter: false,
            spring: true,
            summer: false,
        },
        formula: 4,
        compost: -8,
        manure: 4,
        color: Color::Fixed(8),
    };
    pub const GARLIC: Self = Self {
        name: "Garlic",
        abbrev: "Ga",
        seasons: Season {
            autumn: true,
            winter: true,
            spring: true,
            summer: true,
        },
        formula: 4,
        compost: -8,
        manure: 4,
        color: Color::Fixed(230),
    };
    pub const ONION: Self = Self {
        name: "Onion",
        abbrev: "On",
        seasons: Season {
            autumn: true,
            winter: false,
            spring: true,
            summer: true,
        },
        formula: -8,
        compost: 4,
        manure: 4,
        color: Color::Fixed(130),
    };
    pub const PEPPER: Self = Self {
        name: "Pepper",
        abbrev: "Pe",
        seasons: Season {
            autumn: true,
            winter: false,
            spring: false,
            summer: true,
        },
        formula: 4,
        compost: 4,
        manure: -8,
        color: Color::Fixed(1),
    };
    pub const POMEGRANATE: Self = Self {
        name: "Pomegranate",
        abbrev: "Pg",
        seasons: Season {
            autumn: false,
            winter: false,
            spring: true,
            summer: true,
        },
        formula: -8,
        compost: 4,
        manure: 4,
        color: Color::Fixed(162),
    };
}
