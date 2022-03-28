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
    pub fn new(neuron_count : u32, channel_count: u32) -> NetworkRenderer {
        println!("Creating network renderer  with {} neurons and {} channels", neuron_count, channel_count);
        let width = channel_count;
        let height = neuron_count;

        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title("Network Renderer")
            .with_inner_size(glutin::dpi::LogicalSize::new(width as f64, height as f64));
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

                    uniform uint time;

                    out vec4 color;

                    void main() {
                        color = vec4(1.0, 1.0, 1.0, 1);
                    }
                "
            }
        ).unwrap();

        // Empty 2D 256x256 texture
        let texture = glium::texture::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap,
            neuron_count,
            channel_count,
        );

        NetworkRenderer {
            running: true,
            display,
            event_loop,
            vertex_buffer,
            index_buffer,
            program,
        }
    }

    pub fn render(&self, time: u64) {
        let uniforms = uniform! {
            time: time as u32 // No u64 support. Will cause rendering to be bugged sometimes, but it's not a big deal.
        };

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
