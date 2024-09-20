use crate::{continent_generator::Continent, province_generator::Province};

pub struct World {
    pub seed: u32,
    pub size: (u32, u32),
    pub pixels: Vec<Vec<WorldPixel>>,
    pub continents: Vec<Continent>,
    pub provinces: Vec<Province>,
}

impl World {
    pub fn get_pixel(&self, i: usize, j: usize) -> &WorldPixel {
        &self.pixels[i][j]
    }

    pub fn get_pixels_around(&self, i: usize, j: usize) -> Vec<&WorldPixel> {
        let mut pixels: Vec<&WorldPixel> = Vec::new();
        // Up
        if i != 0 {
            pixels.push(self.get_pixel(i - 1, j))
        }
        // Down
        if i != self.size.0 as usize - 1 {
            pixels.push(self.get_pixel(i + 1, j))
        }
        // Left
        if j != 0 {
            pixels.push(self.get_pixel(i, j - 1))
        }
        // Right
        if j != self.size.1 as usize - 1 {
            pixels.push(self.get_pixel(i, j + 1))
        }
        return pixels
    }

    pub fn get_mut_pixel(&mut self, i: usize, j: usize) -> &mut WorldPixel {
        &mut self.pixels[i][j]
    }
}

pub struct WorldPixel {
    pub position: (u32, u32),
    pub terrain: Terrain,
    pub province_id: Option<u32>,
    pub continent_id: Option<u32>
}

#[derive(Clone)]
pub enum Terrain {
    Plains,
    Mountains,
    Hills,
    Water
}

impl Terrain {
    pub fn to_color(&self) -> Vec<u8>{
        match self {
            Terrain::Plains => {return vec![6, 169, 0]},
            Terrain::Mountains => {return vec![40, 10, 0]},
            Terrain::Hills => {return vec![73, 24, 0]},
            Terrain::Water => {return vec![97, 151, 248]}
        }
    }

    pub fn is_land(&self) -> bool {
        match *self {
            Terrain::Water => {return false}
            _ => {return true}
        }
    }
}

impl PartialEq for Terrain {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Self::Hills => {
                match *other {
                    Terrain::Hills => {return true},
                    _ => {}
                }
            }
            Terrain::Plains => {
                match *other {
                    Terrain::Plains => {return true},
                    _ => {}
                }
            },
            Terrain::Mountains => {
                match *other {
                    Terrain::Mountains => {return true},
                    _ => {}
                }
            },
            Terrain::Water => {
                match *other {
                    Terrain::Water => {return true},
                    _ => {}
                }
            }
        }
        return false
    }
}