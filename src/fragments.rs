use core::f32;

use nalgebra_glm::{dot, Vec2, Vec3};
use crate::screen::color::Color;
use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Fragment {
    pub position: Vec2,
    pub color: Color,
    pub depth: f32,
}


impl Fragment {
    pub fn new(x: f32, y: f32, color: Color, depth: f32) -> Self {
        Fragment {
            position: Vec2::new(x, y),
            color,
            depth,
        }
    }
}

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment>{
    let line_color = Color::new(0xff, 0xff, 0xff);
    let mut fragments = Vec::new();


    let start = a.transformed_position;
    let end = b.transformed_position;
    
    // Bresenham's algorithm
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let mut x1 = end.x as i32;
    let mut y1 =end.y as i32;

    let dx = (x1-x0).abs();
    let dy = (y1-y0).abs();

    let mut sx = if x0<x1 {1} else {-1};
    let mut sy = if y0<y1 {1} else {-1};

    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };
    loop {
        let z = start.z + (end.z - start.z) * (x0 - start.x as i32) as f32 / (end.x - start.x) as f32;
        fragments.push(Fragment::new(x0 as f32, y0 as f32, line_color, z));

        if x0 == x1 && y0 == y1 { break; }

        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }

    fragments
}
