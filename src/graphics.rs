//! Abstractions for the OpenGL graphics pipeline

#[derive(Debug, Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

use glium::VertexBuffer;
use glium::index::NoIndices;
use glium::Program;
use glium::Surface;

pub struct Quad<'window> {
    vs: [Vertex; 4],
    display: &'window Facade,
    vertices: VertexBuffer<Vertex>,
    indices: NoIndices,
    program: Program,
}

use glium::backend::Facade;
type Coord = (i32, i32);
type Size = (i32, i32);
use config::Config;

impl<'window> Quad<'window> {
    pub fn new<F: Facade>(display: &'window F, config: Config, origin: Coord, size: Size) -> Self {
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

        let vert = r#"
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let frag = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        Quad {
            display: display,
            vertices: VertexBuffer::new(display, &vertices).unwrap(),
            indices: NoIndices(PrimitiveType::TriangleStrip),
            program: Program::from_source(display, vert, frag, None).unwrap(),
            vs: vertices,
        }
    }

    pub fn render<S: Surface>(&self, target: &mut S) {
        use glium::uniforms::EmptyUniforms;

        target.draw(&self.vertices, &self.indices, &self.program, &EmptyUniforms, &Default::default()).unwrap();
    }
}

pub fn pixel_to_unit(pixel: i32, bound: u32) -> f32 {
    let origin = (bound as f32) / 2f32;
    (pixel as f32 - origin) / origin
}

pub trait Render {
    fn render<'window, 'entity, R: Renderable<'window, 'entity> + 'entity + 'window>(&mut self, renderable: &'entity R);
}

use glium::Frame;

impl Render for Frame {
    fn render<'window, 'entity, R: Renderable<'window, 'entity> + 'entity + 'window>(&mut self, renderable: &'entity R) {
        use glium::uniforms::EmptyUniforms;
        use glium::index::PrimitiveType;

        let vert = r#"
            #version 140

            in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        "#;

        let frag = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;

        // let display = renderable.display();
        // let vertices = VertexBuffer::new(display, renderable.vertices()).unwrap();
        let indices = NoIndices(PrimitiveType::TriangleStrip);
        let vertices = renderable.vertices2();
        //let program = Program::from_source(display, vert, frag, None).unwrap();

        self.draw(vertices, indices, renderable.program(), &EmptyUniforms, &Default::default()).unwrap();
    }
}

use glium::Display;

trait Renderable<'window, 'entity> {
    fn vertices(&'entity self) -> &'entity [Vertex];
    fn vertices2(&'entity self) -> &'entity VertexBuffer<Vertex>;
    fn display(&'window self) -> &'window Facade;
    fn program(&'entity self) -> &'entity Program;
}

impl<'window, 'entity> Renderable<'window, 'entity> for Quad<'window> {
    fn vertices(&'entity self) -> &'entity [Vertex] {
        &self.vs
    }

    fn vertices2(&'entity self) -> &'entity VertexBuffer<Vertex> {
        &self.vertices
    }

    fn display(&'window self) -> &'window Facade {
        self.display
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
