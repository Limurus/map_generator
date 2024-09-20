use std::collections::HashMap;

use rand::{self, Rng, thread_rng};

use crate::{Settings, World, WorldPixel};


#[derive(Clone)]
pub struct Province {
    pub id: u32,
    pub continent_id: Option<u32>,
    pub elements: Vec<(u32, u32)>,
    pub color: (u8, u8, u8),
    pub is_land: bool,
    pub neighbors: Vec<u32>,
    pub resources: HashMap<String, u32>
}

impl Province {
    pub fn new(id: u32, color: (u8, u8, u8), is_land: bool) -> Self {
        Province { id, elements: Vec::new(), continent_id: None, color, is_land, neighbors: Vec::new(), resources: HashMap::new() }
    }

    pub fn add_pixel(&mut self, pixel: &mut WorldPixel) {
        pixel.province_id = Some(self.id);
        self.elements.push(pixel.position);
    }

    pub fn add_province_neighbor(&mut self, neighbor_province: &mut Province) {
        self.neighbors.push(neighbor_province.id);
        neighbor_province.neighbors.push(self.id);
    }

    pub fn set_resource(&mut self, resource_name: &str, value: u32) {
        if self.resources.contains_key(resource_name) {
            panic!("Resource {resource_name:} is already exist in {:}", self.id)
        }
        self.resources.insert(resource_name.to_string(), value);
    }
}

pub fn generate_provinces(world: &mut World, settings: &Settings, colors: Vec<(u8, u8, u8)>) -> Vec<Province> {
    let mut random = thread_rng();
    let mut colors = colors;
    let mut provinces: Vec<Province> = Vec::new();
    let mut current_id = 0;
    for continent in world.continents.clone() {
        let mut unchecked_pixels: HashMap<(u32, u32), bool> = HashMap::new();
        unchecked_pixels.extend(continent.members.clone().iter().map(|element| (*element, true)));
        while !unchecked_pixels.is_empty() {
            let color_pos = random.gen_range(0..colors.len());
            let random_pixel = unchecked_pixels.keys().skip(random.gen_range(0..unchecked_pixels.keys().len())).next().unwrap();
            let mut province = generate_land_province(current_id, random_pixel.clone(), settings.max_province_size, world, continent.id, colors[color_pos].clone(), settings.min_province_size);
            for pixel in province.elements.iter() {
                unchecked_pixels.remove(pixel);
            }
            if (province.elements.len() as u32) < settings.min_province_size {
                let neighbors: HashMap<u32, u32> = found_province_neighbor(&province, world).iter().map(|id| {
                    (*id, provinces.get(*id as usize).unwrap().elements.len() as u32)
                }).collect();
                let minimal = *neighbors.values().min().unwrap();
                let possible_province: Vec<(&u32, &u32)> = neighbors.iter().filter(|(_id, length)| {
                    **length == minimal
                }).collect();
                let new_province = provinces.get_mut(*(possible_province.get(random.gen_range(0..possible_province.len())).unwrap().0) as usize).unwrap();
                for pixel in province.elements.iter() {
                    new_province.add_pixel(world.get_mut_pixel(pixel.1 as usize, pixel.0 as usize));
                }
                continue;
            }
            colors.swap_remove(color_pos);
            
            let neighbors = found_province_neighbor(&province, &world);
            for neighbor in neighbors {
                province.add_province_neighbor(provinces.get_mut(neighbor as usize).unwrap());
            }
            provinces.push(province);
            current_id += 1;
        }
    }

    return provinces;
}

pub fn found_province_neighbor(province: &Province, world: &World) -> Vec<u32> {
    let mut provinces: Vec<u32> = Vec::new();
    for pixel in province.elements.iter() {
        for neighbors in world.get_pixels_around(pixel.1 as usize, pixel.0 as usize).iter() {
            if neighbors.province_id.is_some() && neighbors.province_id.unwrap() != province.id && !provinces.contains(&neighbors.province_id.unwrap()) {
                provinces.push(neighbors.province_id.unwrap());
            }
        }
    }
    return provinces
}

pub fn generate_land_province(id: u32, first_pixel: (u32, u32), max_size: u32, world: &mut World, continent_id: u32, color: (u8, u8, u8), min_size: u32) -> Province {
    let mut random = thread_rng();
    let mut province = Province::new(id, color, true);
    let mut province_size = 0;
    let mut possible_pixels: HashMap<(u32, u32), u8> = HashMap::new();
    let mut anyway_added: Vec<(u32, u32)> = Vec::new();
    let mut added_pixels: Vec<(u32, u32)> = Vec::new();
    let result_province_size = random.gen_range(((max_size - min_size).max(min_size))..max_size);
    anyway_added.push(first_pixel);
    loop {
        if !anyway_added.is_empty() {
            for pixel in anyway_added.iter() {
                province.add_pixel(world.get_mut_pixel(pixel.1 as usize, pixel.0 as usize));
                added_pixels.push(*pixel);
                match possible_pixels.get(pixel) {
                    Some(_) => {possible_pixels.remove(pixel);},
                    None => {}
                };
            }
            province_size += anyway_added.len() as u32;
            anyway_added.clear();
        } else if province_size >= result_province_size || possible_pixels.is_empty() {
            break;
        } else {
            let mut variants = Vec::new();
            for (position, count) in possible_pixels.iter() {
                variants.extend(std::iter::repeat(position).take((count.clone() + (count.clone() - 1) * 2) as usize));
            }
            let added_pixel: (u32, u32) = variants[random.gen_range(0..(variants.len()))];
            province.add_pixel(world.get_mut_pixel(added_pixel.1 as usize, added_pixel.0 as usize));
            possible_pixels.remove(&added_pixel);
            added_pixels.push(added_pixel);
            province_size += 1;
        }
        for position in added_pixels.iter() {
            let around = world.get_pixels_around(position.1 as usize, position.0 as usize);
            for pixel in around {
                if pixel.terrain.is_land() && pixel.continent_id.unwrap() == continent_id && pixel.province_id.is_none() {
                    match possible_pixels.get_mut(&pixel.position) {
                        Some(value) => {
                            *value += 1;
                        },
                        None => {possible_pixels.insert(pixel.position, 1);}
                    }
                } else {
                    possible_pixels.remove(position);
                }
            }
        }
        for (possible_pixel, pixel_count) in possible_pixels.iter() {
            let around_pixel = world.get_pixels_around(possible_pixel.1 as usize, possible_pixel.0 as usize); 
            if around_pixel.iter().filter(|pixel| pixel.terrain.is_land() && pixel.continent_id.unwrap() == continent_id).count() as u8 == *pixel_count {
                anyway_added.push(*possible_pixel);
            }
        }
        added_pixels.clear()
    }
    
    return province;
}
