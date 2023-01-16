#![warn(clippy::pedantic, clippy::nursery, rust_2018_idioms)]

use anyhow::Context;
use dialoguer::{theme::ColorfulTheme, Input};
use plant::{CropRatios, FarmSize, Plant, Seasons};

mod lookup;
mod plant;

#[derive(Debug, Clone)]
struct SeedCounts {
    carrot: u32,
    corn: u32,
    dragon_fruit: u32,
    durian: u32,
    eggplant: u32,
    pomegranate: u32,
    pumpkin: u32,
    watermelon: u32,
    asparagus: u32,
    toma_root: u32,
    potato: u32,
    onion: u32,
    pepper: u32,
    garlic: u32,
}

impl SeedCounts {
    fn get_mut_from_name<'a>(&'a mut self, name: &str) -> anyhow::Result<&'a mut u32> {
        Ok(match name {
            "Carrot" => &mut self.carrot,
            "Corn" => &mut self.corn,
            "Dragon Fruit" => &mut self.dragon_fruit,
            "Durian" => &mut self.durian,
            "Eggplant" => &mut self.eggplant,
            "Pomegranate" => &mut self.pomegranate,
            "Pumpkin" => &mut self.pumpkin,
            "Watermelon" => &mut self.watermelon,
            "Asparagus" => &mut self.asparagus,
            "Toma Root" => &mut self.toma_root,
            "Potato" => &mut self.potato,
            "Onion" => &mut self.onion,
            "Pepper" => &mut self.pepper,
            "Garlic" => &mut self.garlic,
            _ => Err(anyhow::anyhow!("Invalid plant name {}", name))?,
        })
    }

    fn get_val_from_name(&self, name: &str) -> anyhow::Result<u32> {
        Ok(match name {
            "Carrot" => self.carrot,
            "Corn" => self.corn,
            "Dragon Fruit" => self.dragon_fruit,
            "Durian" => self.durian,
            "Eggplant" => self.eggplant,
            "Pomegranate" => self.pomegranate,
            "Pumpkin" => self.pumpkin,
            "Watermelon" => self.watermelon,
            "Asparagus" => self.asparagus,
            "Toma Root" => self.toma_root,
            "Potato" => self.potato,
            "Onion" => self.onion,
            "Pepper" => self.pepper,
            "Garlic" => self.garlic,
            _ => Err(anyhow::anyhow!("Invalid plant name {}", name))?,
        })
    }
}

fn prompt_for_seed(name: [&str; 2], plant: &Plant, season: &Seasons) -> anyhow::Result<u32> {
    if !plant.in_season(season) {
        return Ok(0);
    }
    Input::<u32>::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Enter number of {} Seeds ({})", name[0], name[1]))
        .default(0)
        .interact_text()
        .with_context(|| format!("Couldn't parse {} seeds", name[0]))
}

fn get_seed_counts(season: &Seasons) -> anyhow::Result<SeedCounts> {
    Ok(SeedCounts {
        carrot: prompt_for_seed(["Carrot", "Oblong Seeds"], &Plant::CARROT, season)?,
        corn: prompt_for_seed(["Corn", "Clustered Seeds"], &Plant::CORN, season)?,
        dragon_fruit: prompt_for_seed(
            ["Dragon Fruit", "Bulbous Seeds"],
            &Plant::DRAGON_FRUIT,
            season,
        )?,
        durian: prompt_for_seed(["Durian", "Brittle Seed Pods"], &Plant::DURIAN, season)?,
        eggplant: prompt_for_seed(["Eggplant", "Swirly Seeds"], &Plant::EGGPLANT, season)?,
        pomegranate: prompt_for_seed(
            ["Pomegranate", "Windblown Seeds"],
            &Plant::POMEGRANATE,
            season,
        )?,
        pumpkin: prompt_for_seed(["Pumpkin", "Sharp Seeds"], &Plant::PUMPKIN, season)?,
        watermelon: prompt_for_seed(["Watermelon", "Square Seeds"], &Plant::WATERMELON, season)?,
        asparagus: prompt_for_seed(["Asparagus", "Tubular Seeds"], &Plant::ASPARAGUS, season)?,
        toma_root: prompt_for_seed(["Toma Root", "Spiky Seeds"], &Plant::TOMA_ROOT, season)?,
        potato: prompt_for_seed(["Potato", "Fluffy Seeds"], &Plant::POTATO, season)?,
        onion: prompt_for_seed(["Onion", "Pointy Seeds"], &Plant::ONION, season)?,
        pepper: prompt_for_seed(["Pepper", "Lumpy Seeds"], &Plant::PEPPER, season)?,
        garlic: prompt_for_seed(["Garlic", "Seed Pods"], &Plant::GARLIC, season)?,
    })
}

fn get_crop_ratio(size: &FarmSize, season: &Seasons) -> anyhow::Result<CropRatios> {
    let layouts_size = match (size.width, size.height) {
        (w, h) if w % 2 == 0 && h % 2 == 0 => ["1:1", "2:1", "1:1:1", "2:1:1"].as_slice(),
        (w, _) if w % 2 == 0 => ["1:1", "2:1"].as_slice(),
        _ => ["1:1"].as_slice(),
    };
    let layouts = match season {
        // All combinations are supported in autumn and spring
        Seasons::Autumn | Seasons::Spring => layouts_size,
        // Only 1:1:1 is supported in winter
        Seasons::Winter if layouts_size.contains(&"1:1:1") => ["1:1:1"].as_slice(),
        Seasons::Winter => &[],
        // 1:1 not supported in summer, so remove
        Seasons::Summer => &layouts_size[1..],
    };
    anyhow::ensure!(!layouts.is_empty(), "No available crop layouts");
    let layout_idx = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter preferred crop layout")
        .default(0)
        .items(layouts)
        .interact()
        .context("Couldn't get crop layout")?;
    layouts[layout_idx].parse::<plant::CropRatios>()
}

fn get_season() -> anyhow::Result<Seasons> {
    let seasons = &["Autumn", "Winter", "Spring", "Summer"];
    let season_idx = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter current season")
        .default(0)
        .items(seasons)
        .interact()
        .context("Couldn't get season")?;
    seasons[season_idx].parse::<plant::Seasons>()
}

fn get_farm_size() -> anyhow::Result<FarmSize> {
    let validator = |input: &u16| -> Result<(), &str> {
        if *input == 0 {
            Err("Input cannot be 0")
        } else {
            Ok(())
        }
    };

    let width = Input::<u16>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter farm width")
        .validate_with(validator)
        .interact_text()
        .context("Couldn't parse farm width")?;
    let height = Input::<u16>::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter farm height")
        .validate_with(validator)
        .interact_text()
        .context("Couldn't parse farm height")?;
    Ok(FarmSize { width, height })
}

struct GridZip<T> {
    internal: Vec<T>,
    row: usize,
    row_size: usize,
}

impl<T> Iterator for GridZip<T>
where
    T: Iterator + std::fmt::Debug,
    <T as Iterator>::Item: std::fmt::Debug,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self
            .internal
            .iter_mut()
            .skip(self.row * self.row_size)
            .take(self.row_size)
            .map(Iterator::next)
            .collect::<Option<Vec<_>>>();
        if val.is_none() {
            self.row += 1;
            // Recursion, woo!
            return self.next();
        }
        if let Some(unwrapped) = &val {
            if unwrapped.is_empty() {
                return None;
            }
        }
        val
    }
}

impl<T> GridZip<T> {
    fn new(internal: Vec<T>, row_size: usize) -> Self {
        Self {
            internal,
            row: 0,
            row_size,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let season = get_season()?;
    let size = get_farm_size()?;
    let ratio = get_crop_ratio(&size, &season)?;
    let mut seeds = get_seed_counts(&season)?;

    let mut planted_seeds = Vec::new();

    let crop_seed_min_num = ratio.get_min_seeds_per_crop();

    let mut layouts = lookup::get_combos(&season, &ratio)
        .context("No crops for current season and layout")?
        .to_vec();
    fastrand::shuffle(&mut layouts);

    let mut retain = vec![true; layouts.len()];

    for _ in 0..ratio.get_filled_size(&size) {
        'outer: for (idx, &i) in layouts.iter().enumerate() {
            let mut seeds_tmp = seeds.clone();
            for j in i {
                let current_crop = seeds_tmp.get_val_from_name(j.name)?;
                if current_crop >= crop_seed_min_num {
                    *seeds_tmp.get_mut_from_name(j.name)? = current_crop - crop_seed_min_num;
                } else {
                    retain[idx] = false;
                    continue 'outer;
                }
            }
            planted_seeds.push(i);
            seeds = seeds_tmp;
            break;
        }
    }

    let mut grid = Vec::new();

    for i in 0..ratio.get_filled_size(&size) {
        if let Some(x) = planted_seeds.get((i) as usize) {
            grid.push(match ratio {
                CropRatios::OneOne => format!("╔═══╤═══╤═══╗\n║ {0} │ {0} │ {0} ║\n╟───┼───┼───╢\n║ {0} │   │ {1} ║\n╟───┼───┼───╢\n║ {1} │ {1} │ {1} ║\n╚═══╧═══╧═══╝", x[0].color.paint("█"), x[1].color.paint("█")),
                CropRatios::OneOneOne => format!("╔═══╤═══╤═══╤═══╤═══╤═══╗\n║ {0} │ {1} │ {1} ┃ {1} │ {1} │ {0} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │ {1} │ {2} ┃ {2} │ {1} │ {0} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │ {2} │ {2} ┃ {2} │ {2} │ {0} ║\n╟━━━┿━━━┿━━━╋━━━┿━━━┿━━━╢\n║ {0} │ {2} │ {2} ┃ {2} │ {2} │ {0} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │ {1} │ {2} ┃ {2} │ {1} │ {0} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │ {1} │ {1} ┃ {1} │ {1} │ {0} ║\n╚═══╧═══╧═══╧═══╧═══╧═══╝", x[0].color.paint("█"), x[1].color.paint("█"), x[2].color.paint("█")),
                CropRatios::TwoOne => format!("╔═══╤═══╤═══╤═══╤═══╤═══╗\n║ {0} │ {0} │ {1} ┃ {1} │ {2} │ {2} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │ {0} │ {1} ┃ {1} │ {2} │ {2} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │ {0} │ {1} ┃ {1} │ {2} │ {2} ║\n╚═══╧═══╧═══╧═══╧═══╧═══╝", x[0].color.paint("█"), x[2].color.paint("█"), x[1].color.paint("█")),
                CropRatios::TwoOneOne => format!("╔═══╤═══╤═══╤═══╤═══╤═══╗\n║ {0} │ {0} │ {1} ┃ {1} │ {3} │ {3} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │   │ {1} ┃ {1} │   │ {3} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │ {2} │ {2} ┃ {2} │ {2} │ {3} ║\n╟━━━┿━━━┿━━━╋━━━┿━━━┿━━━╢\n║ {0} │ {2} │ {2} ┃ {2} │ {2} │ {3} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │   │ {1} ┃ {1} │   │ {3} ║\n╟───┼───┼───╂───┼───┼───╢\n║ {0} │ {0} │ {1} ┃ {1} │ {3} │ {3} ║\n╚═══╧═══╧═══╧═══╧═══╧═══╝", x[0].color.paint("█"), x[2].color.paint("█"), x[3].color.paint("█"), x[1].color.paint("█"))
            });
        } else {
            grid.push(match ratio {
                CropRatios::OneOne => "╔═══╤═══╤═══╗\n║   │   │   ║\n╟───┼───┼───╢\n║   │   │   ║\n╟───┼───┼───╢\n║   │   │   ║\n╚═══╧═══╧═══╝".to_string(), 
                CropRatios::OneOneOne | CropRatios::TwoOneOne => "╔═══╤═══╤═══╤═══╤═══╤═══╗\n║   │   │   ┃   │   │   ║\n╟───┼───┼───╂───┼───┼───╢\n║   │   │   ┃   │   │   ║\n╟───┼───┼───╂───┼───┼───╢\n║   │   │   ┃   │   │   ║\n╟━━━┿━━━┿━━━╋━━━┿━━━┿━━━╢\n║   │   │   ┃   │   │   ║\n╟───┼───┼───╂───┼───┼───╢\n║   │   │   ┃   │   │   ║\n╟───┼───┼───╂───┼───┼───╢\n║   │   │   ┃   │   │   ║\n╚═══╧═══╧═══╧═══╧═══╧═══╝".to_string(), 
                CropRatios::TwoOne => "╔═══╤═══╤═══╤═══╤═══╤═══╗\n║   │   │   ┃   │   │   ║\n╟───┼───┼───╂───┼───┼───╢\n║   │   │   ┃   │   │   ║\n╟───┼───┼───╂───┼───┼───╢\n║   │   │   ┃   │   │   ║\n╚═══╧═══╧═══╧═══╧═══╧═══╝".to_string()
            });
        }
    }

    let grid = grid.iter().map(|x| x.lines()).collect();

    for i in GridZip::new(grid, ratio.get_filled_size_horizontal(&size) as usize) {
        println!("{}", i.join(" "));
    }

    let mut used = Vec::new();
    for i in planted_seeds {
        for j in i {
            if !used.contains(&j) {
                used.push(j);
                println!("{}", j.color.paint(j.name));
            }
        }
    }

    Ok(())
}
