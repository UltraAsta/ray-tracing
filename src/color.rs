use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

// Write the translated [0, 255] value of each color component
pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    // 255.999 instead of 255.0 is used to solve floting point precision related problems, it's basically a safety margin
    let r = (255.999 * pixel_color.x()) as i32;
    let g = (255.999 * pixel_color.y()) as i32;
    let b = (255.999 * pixel_color.z()) as i32;

    writeln!(out, "{} {} {}", r, g, b).expect("writing colors")
}
