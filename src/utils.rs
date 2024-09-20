use std::fs::OpenOptions;
use std::io::Write;

use crate::world::World;
use crate::START_TIME;


pub fn world_to_image(world: &World) {
    let mut colors: Vec<u8> = Vec::new();
    for row in world.pixels.iter() {
        let colors_row = row.iter().map(|pixel| pixel.terrain.to_color());
        for pixel in colors_row {
            colors.extend(pixel);
        }
    }
    buffer_to_image(&format!("logs/{}/maps/terrain.png", START_TIME.as_str()), colors, world.size);
}

pub fn continent_to_image(world: &World) {
    let mut continents_image: Vec<u8> = Vec::new();
    for i  in world.pixels.iter() {
        for j in i.iter() {
            match j.continent_id {
                Some(id) => {
                    let color = world.continents.get(id as usize).unwrap().color;
                    continents_image.extend(&mut vec![color.0, color.1, color.2].iter());
                },
                None => {
                    continents_image.extend(vec![0, 0, 0]);
                }
            }
        }
    }
    buffer_to_image(&format!("logs/{}/maps/continents.png", START_TIME.as_str()), continents_image, world.size);
}

pub fn province_to_image(world: &World) {
    let mut buff = Vec::new();
    for i  in world.pixels.iter() {
        for j in i.iter() {
            match j.province_id {
                Some(id) => {
                    let province_color = world.provinces.get(id as usize).unwrap().color;
                    buff.extend(&mut vec![province_color.0, province_color.1, province_color.2].iter());
                },
                None => {
                    buff.extend(vec![0, 0, 0]);
                }
            }
        }
    }
    buffer_to_image(&format!("logs/{}/maps/provinces.png", START_TIME.as_str()), buff, world.size)
}

pub fn resource_to_image(world: &World, resource: &str) {
    let mut buff = Vec::new();
    for i  in world.pixels.iter() {
        for j in i.iter() {
            match j.province_id {
                Some(id) => {
                    let resource: u8 = (*world.provinces.get(id as usize).unwrap().resources.get(resource).unwrap()).clamp(0, 255) as u8;
                    buff.extend(&mut vec![0, resource, 0].iter());
                },
                None => {
                    buff.extend(vec![0, 0, 0]);
                }
            }
        }
    }
    buffer_to_image(&format!("logs/{}/maps/{resource}.png", START_TIME.as_str()), buff, world.size)
}

pub fn buffer_to_image(result_file_name: &str, buff: Vec<u8>, size: (u32, u32)) {
    image::save_buffer(
        result_file_name,
        &buff,
        size.0 as u32,
        size.1 as u32,
        image::ColorType::Rgb8)
        .expect("");
}

pub fn log(data: &str) {
    let data = format!("{}\n", data);
    let mut log_file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(format!("logs/{}/log.txt", START_TIME.as_str()))
        .unwrap();
    log_file.write_all(data.as_bytes()).unwrap();
}

pub fn generate_colors() -> Vec<(u8, u8, u8)> {
    let mut colors: Vec<(u8, u8, u8)> = Vec::new();
    for i in 0..50 {
        for j in 0..50 {
            for t in 0..50 {
                colors.push((30 + i * 4, 30 + j * 4, 30 + t * 4))
            }
        }
    }
    return colors
}

pub mod necessary {
    use std::path::Path;
    use std::fs::{create_dir, File};

    use crate::{log, START_TIME};

    // By default - create necessary files/dirs and log creation
    // TODO: Make in settings some options for this
    pub fn check_necessary_files() {
        let log_dir = START_TIME.as_str();
        check_dir("logs");
        check_dir(&format!("logs/{}/", log_dir));
        check_file(&format!("logs/{}/log.txt", log_dir));
        check_dir(&format!("logs/{}/maps/", log_dir));
        check_dir("gamedata");
        check_file("gamedata/settings.txt");
    }

    fn check_dir(dir_name: &str) {
        if !Path::new(dir_name).exists() {
            match create_dir(dir_name) {
                Ok(_) => {},
                Err(err) => {panic!("{:?}\n{:?}", err, dir_name)}
            };
        }
    }
    
    fn check_file(file_name: &str) {
        if !Path::new(file_name).exists() {
            create_file(file_name);
            log(format!("[Info] {:?} - created", file_name).as_str());
        }
    }
    
    fn create_file(file_name: &str) {
        match File::create(file_name) {
            Ok(_) => {},
            Err(err) => {log(format!("[Error] File {:?} - {:?}", file_name, err).as_str())}
        };
    }
}