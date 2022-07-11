// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let total_produce: f64 = speed as f64 * 221_f64;
    let successful_produce: f64 = match speed {
        1..=4 => total_produce,
        5..=8 => total_produce * 0.9,
        9 | 10 => total_produce * 0.77,
        _ => 0.0_f64,
    };
    successful_produce
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0) as u32
}
