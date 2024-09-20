pub struct Settings {
    pub world_size: (u32, u32),
    pub seed: u32,
    pub min_province_size: u32,
    pub max_province_size: u32,
    pub min_water_province_size: u32,
    pub max_water_province_size: u32
}

impl Settings {
    pub fn new(world_size: (u32, u32), seed: u32, max_land_size: u32, min_land_size: u32, max_water_size: u32, min_water_size: u32) -> Self {
        Settings {
            seed: seed,
            world_size: world_size,
            max_province_size: max_land_size,
            min_province_size: min_land_size,
            max_water_province_size: max_water_size,
            min_water_province_size: min_water_size
        }
    }
}