use std::time::Instant;
use noise::{self, utils::{NoiseMap, NoiseMapBuilder, PlaneMapBuilder}, Billow, Perlin};
use chrono::offset::Local;
use lazy_static::lazy_static;

pub mod continent_generator;
pub mod province_generator;
pub mod world;
pub mod settings;
pub mod utils;

use continent_generator::generate_continents;
use province_generator::{Province, generate_provinces};
use world::{World, WorldPixel, Terrain};
use settings::Settings;
use utils::*;


lazy_static! {
    pub static ref START_TIME: String = Local::now().format("%Y-%m-%d_%H-%M").to_string();
}


macro_rules! time {
    ($x:expr) => {{
        let start = Instant::now();
        let result = $x;
        log(format!("{:?}", start.elapsed()).as_str());
        result
    }};
}


pub fn generate_world(settings: Settings) -> World {
    necessary::check_necessary_files();
    let colors = generate_colors();
    let mut world = time!(generate_map(&settings));
    world_to_image(&world);
    let continents = time!(generate_continents(&mut world, settings.min_province_size.clone(), colors.clone()));
    world.continents = continents;
    continent_to_image(&world);
    let provinces = time!(generate_provinces(&mut world, &settings, colors.clone()));
    world.provinces = provinces;
    province_to_image(&world);
    return world
}

fn generate_noise(seed: u32, size: (u32, u32)) -> NoiseMap {
    let noises = Billow::<Perlin>::new(seed);
    PlaneMapBuilder::<_, 2>::new(&noises)
        .set_size(size.0 as usize, size.1 as usize)
        .build()
}

// For future
// TODO: Use this in water province generation
fn _delete_single_water_pixel(world: &mut World) {
    let mut water_pixels: Vec<(u32, u32)> = Vec::new();
    for row in world.pixels.iter() {
        for pixel in row.iter() {
            if !pixel.terrain.is_land() {
                water_pixels.push(pixel.position);
            }
        }
    }
    for pixel in water_pixels {
        if world.get_pixels_around(pixel.1 as usize, pixel.0 as usize).iter().filter(|pixel| !pixel.terrain.is_land()).count() == 0 {
            world.get_mut_pixel(pixel.1 as usize, pixel.0 as usize).terrain = Terrain::Plains;
        }
    }
}

fn generate_map(settings: &Settings) -> World {
    let terrain_map = generate_noise(settings.seed, settings.world_size);
    let mut terrain_map = terrain_map.iter();
    let mut pixels: Vec<Vec<WorldPixel>> = Vec::new();
    for i in 0..settings.world_size.0 {
        let mut pixel_row: Vec<WorldPixel> = Vec::new();
        for j in 0..settings.world_size.1 {
            let terrain;
            let biome_value = *terrain_map.next().expect("");
            if biome_value >= 0.8 {
                terrain = Terrain::Mountains;
            } else if biome_value >= 0.7 {
                terrain = Terrain::Hills
            } else if biome_value <= -0.75 {
                terrain = Terrain::Water;
            } else {
                terrain = Terrain::Plains;
            }
            let pixel = WorldPixel {
                position: (j, i),
                province_id: None,
                continent_id: None,
                terrain: terrain
            };
            pixel_row.push(pixel)
        }
        pixels.push(pixel_row);
    }
    World {
        seed: settings.seed,
        size: settings.world_size,
        pixels: pixels,
        continents: Vec::new(),
        provinces: Vec::new()
    }
}


pub fn run() {
    let settings = Settings::new(
        (1024, 1024),
        124122,
        100,
        25,
        400,
        100
    );
    let _world = time!(generate_world(settings));
}