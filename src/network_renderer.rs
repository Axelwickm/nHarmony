use glium::index::PrimitiveType;
#[allow(unused_imports)]
use glium::{glutin, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

pub struct NetworkRenderer {
    running: bool,
    display: glium::Display,
    event_loop: glutin::event_loop::EventLoop<()>,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    index_buffer: glium::IndexBuffer<u16>,
    program: glium::Program,
}

impl<'a> NetworkRenderer {
    pub fn new() -> NetworkRenderer {
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new();
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        println!("OpenGL version: {:?}", display.get_opengl_version());

        let vertex_buffer = {
            glium::VertexBuffer::new(
                &display,
                &[
                    Vertex {
                        position: [-1.0, -1.0],
                    },
                    Vertex {
                        position: [-1.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, 1.0],
                    },
                    Vertex {
                        position: [1.0, -1.0],
                    },
                ],
            )
            .unwrap()
        };

        let index_buffer =
            glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2, 0, 2, 3])
                .unwrap();

        let program = program!(
            &display,
            140 => {
                vertex: "
                    #version 140

                    in vec2 position;

                    void main() {
                        gl_Position = vec4(position, 0.0, 1.0);
                    }
                ",
                fragment: "
                    #version 140

                    out vec4 color;

                    void main() {
                        color = vec4(1.0, 1.0, 1.0, 1);
                    }
                "
            }
        ).unwrap();

        NetworkRenderer {
            running: true,
            display,
            event_loop,
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    pub fn render(&self) {
        let uniforms = uniform! {};

        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target
            .draw(
                &self.vertex_buffer,
                &self.index_buffer,
                &self.program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        // Event loop
        /*self.event_loop.poll_events(|event| {
            match event {
                _ => (),
            }
        });*/
    }
}
