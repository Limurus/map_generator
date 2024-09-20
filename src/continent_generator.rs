use std::collections::HashMap;
use rand::{thread_rng, Rng};

use crate::{log, Province, World, WorldPixel};


#[derive(Clone)]
pub struct Continent {
    pub id: u32,
    pub members: Vec<(u32, u32)>,
    pub provinces: Vec<Province>,
    pub color: (u8, u8, u8)
}

impl Continent {
    pub fn new(id: u32, color: (u8, u8, u8)) -> Self {
        Continent {
            id: id,
            members: Vec::new(),
            provinces: Vec::new(),
            color: color
        }
    }

    pub fn add_continent_to_pixels(&mut self, world: &mut World) {
        for pixel in self.members.iter() {
            world.get_mut_pixel(pixel.1 as usize, pixel.0 as usize).continent_id = Some(self.id);
        }
    }

    pub fn add_pixel(&mut self, pixel: &WorldPixel) {
        self.members.push(pixel.position);
    }

    pub fn add_province(&mut self, province: &mut Province) {
        province.continent_id = Some(self.id);
        self.provinces.push(province.clone());
    }
}


pub fn generate_continents(world: &mut World, min_province_size: u32, colors: Vec<(u8, u8, u8)>) -> Vec<Continent> {
    let mut colors = colors;
    let mut random = thread_rng();
    let mut continents: Vec<Continent> = Vec::new();
    let mut checked_points: HashMap<(u32, u32), bool> = HashMap::new();
    for i in 0..world.size.0 {
        for j in 0..world.size.1 {
            if checked_points.get(&(i as u32, j as u32)).is_none() {
                if world.get_pixel(i as usize, j as usize).terrain.is_land() {
                    let color_pos = random.gen_range(0..colors.len());
                    let mut continent = Continent::new(continents.len() as u32, colors[color_pos]);
                    colors.swap_remove(color_pos);
                    continent_find(&mut checked_points, world, (i, j), &mut continent);
                    continents.push(continent);
                } else {
                    checked_points.insert((i as u32, j as u32), true);
                }
            }
        }
    }

    // Remove all continents with size less that minimal province_size
    let mut count = 0;
    let mut on_delete = Vec::new();
    let mut current_id = 0;
    for (continent_id, continent) in continents.iter_mut().enumerate() {
        if (continent.members.len() as u32) < min_province_size {
            on_delete.push(continent_id);
            log(format!("Deleted continent - {:?}: {:?} - {:?}", continent_id, continent.color, continent.members.len()).as_str())
        } else {
            continent.id = current_id;
            current_id += 1;
            continent.add_continent_to_pixels(world);
            count += continent.members.len();
            log(format!("Added to map continent - {:?}: {:?} - {:?}", continent_id, continent.color, continent.members.len()).as_str());
        }
    }
    for id in on_delete.iter().rev() {
        continents.remove(*id);
    }
    log(format!("All continent pixels - {:?}\nFrom {:?} possible", count, world.size.0 * world.size.1).as_str());

    return continents
}

fn continent_find(checked_points: &mut HashMap<(u32, u32), bool>, world: &mut World, current_position: (u32, u32), continent: &mut Continent) {
    let world_bounds = (world.size.0 - 1, world.size.1 - 1);
    let mut unchecked_points: HashMap<(u32, u32), bool> = HashMap::new();
    unchecked_points.insert(current_position, true);
    while unchecked_points.len() != 0 {
        let current_position = unchecked_points.keys().last().unwrap().clone();
        checked_points.insert(current_position, true);
        let current_pixel = world.get_pixel(current_position.0 as usize, current_position.1 as usize);
        if !current_pixel.terrain.is_land() {
            unchecked_points.remove(&current_position);
            continue;
        }
        // Up
        if current_position.0 != 0
        && unchecked_points.get(&(current_position.0 - 1, current_position.1)).is_none()
        && checked_points.get(&(current_position.0 - 1, current_position.1)).is_none() {
            unchecked_points.insert((current_position.0 - 1, current_position.1), true);
        }
        // Left
        if current_position.1 != 0
        && unchecked_points.get(&(current_position.0, current_position.1 - 1)).is_none()
        && checked_points.get(&(current_position.0, current_position.1 - 1)).is_none() {
            unchecked_points.insert((current_position.0, current_position.1 - 1), true);
        }
        // Up
        if current_position.0 != (world_bounds.0)
        && unchecked_points.get(&(current_position.0 + 1, current_position.1)).is_none()
        && checked_points.get(&(current_position.0 + 1, current_position.1)).is_none() {
            unchecked_points.insert((current_position.0 + 1, current_position.1), true);
        }
        // Right
        if current_position.1 != (world_bounds.1)
        && unchecked_points.get(&(current_position.0, current_position.1 + 1)).is_none()
        && checked_points.get(&(current_position.0, current_position.1 + 1)).is_none() {
            unchecked_points.insert((current_position.0, current_position.1 + 1), true);
        }
        unchecked_points.remove(&current_position);
        continent.add_pixel(current_pixel);
    }
}