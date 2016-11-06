//! Abstractions for the OpenGL graphics pipeline

use glium::{Frame, Program, Surface, VertexBuffer};
use glium::backend::Facade;
use glium::index::NoIndices;

use config::Config;

type Coord = (i32, i32);
type Size = (i32, i32);

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct Quad {
    vertices: VertexBuffer<Vertex>,
    indices: NoIndices,
    program: Program,
}

impl Quad {
    pub fn new<'window, F: Facade>(display: &'window F, config: Config, origin: Coord, size: Size) -> Self {
        use glium::index::PrimitiveType;

        let p2u = pixel_to_unit;
        let width = config.window_width;
        let height = config.window_height;

        let vertices = [
            Vertex { position: [p2u(origin.0, width), p2u(height as i32 - origin.1, height)] },
            Vertex { position: [p2u(origin.0 + size.0, width), p2u(height as i32 - origin.1, height)] },
            Vertex { position: [p2u(origin.0, width), p2u(height as i32 - origin.1 - size.1, height)] },
            Vertex { position: [p2u(origin.0 + size.0, width), p2u(height as i32 - origin.1 - size.1, height)] },
        ];

        Quad {
            vertices: VertexBuffer::new(display, &vertices).unwrap(),
            indices: NoIndices(PrimitiveType::TriangleStrip),
            program: Program::from_source(display, vertex_shader(), fragment_shader(), None).unwrap(),
        }
    }
}

pub fn pixel_to_unit(pixel: i32, bound: u32) -> f32 {
    let origin = (bound as f32) / 2f32;
    (pixel as f32 - origin) / origin
}

fn vertex_shader() -> &'static str {
    r#"
        #version 140
        in vec2 position;
        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#
}

fn fragment_shader() -> &'static str {
    r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#
}

pub trait Render {
    fn render<'entity, R: Renderable<'entity> + 'entity>(&mut self, renderable: &'entity R);
}

impl Render for Frame {
    fn render<'entity, R: Renderable<'entity> + 'entity>(&mut self, renderable: &'entity R) {
        use glium::uniforms::EmptyUniforms;

        let vertices = renderable.vertices();
        let indices = renderable.indices();
        let program = renderable.program();

        self.draw(vertices, indices, program, &EmptyUniforms, &Default::default()).unwrap();
    }
}

pub trait Renderable<'entity> {
    fn vertices(&'entity self) -> &'entity VertexBuffer<Vertex>;
    fn indices(&'entity self) -> &'entity NoIndices;
    fn program(&'entity self) -> &'entity Program;
}

impl<'entity> Renderable<'entity> for Quad {
    fn vertices(&'entity self) -> &'entity VertexBuffer<Vertex> {
        &self.vertices
    }

    fn indices(&'entity self) -> &'entity NoIndices {
        &self.indices
    }

    fn program(&'entity self) -> &'entity Program {
        &self.program
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_to_unit() {
        let bound = 800;

        assert_eq!(0.5, pixel_to_unit(600, bound));
        assert_eq!(0.0, pixel_to_unit(400, bound));
        assert_eq!(-0.5, pixel_to_unit(200, bound));

        assert_eq!(1.5, pixel_to_unit(1000, bound));
        assert_eq!(-1.5, pixel_to_unit(-200, bound));
    }
}
