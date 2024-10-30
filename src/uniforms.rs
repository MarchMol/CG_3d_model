use nalgebra_glm::{Vec3, Mat4};
use crate::{fragments::Fragment, screen::framebuffer::Framebuffer, shader::vertex_shader, vertex::Vertex};

pub struct Uniforms {
    pub model_matrix: Mat4,
    pub light_dir: Vec3,
}

pub fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // 1. Vertex shader stage
    let mut shaded_vertices: Vec<Vertex> = Vec::new();
    for vertex in vertex_array {
        shaded_vertices.push(vertex_shader(vertex, uniforms))
    }

    // 2. Primitive Assembly stage (only triangles)
    let len = shaded_vertices.len();
    let mut triangles= Vec::new();
    
    for i in (0..len).step_by(3) {
        if i + 2 < len {
            triangles.push([
                shaded_vertices[i].clone(),
                shaded_vertices[i + 1].clone(),
                shaded_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage


    // Fragment Processing Stage

}
