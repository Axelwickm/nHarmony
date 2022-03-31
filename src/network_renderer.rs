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
    texture: glium::texture::Texture2d,
    program: glium::Program,
}

impl<'a> NetworkRenderer {
    pub fn new(neuron_count: u32, channel_count: u32) -> NetworkRenderer {
        println!(
            "Creating network renderer  with {} neurons and {} channels",
            neuron_count, channel_count
        );
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

        let index_buffer = glium::IndexBuffer::new(
            &display,
            PrimitiveType::TrianglesList,
            &[0u16, 1, 2, 0, 2, 3],
        )
        .unwrap();

        let texture = glium::texture::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8,
            glium::texture::MipmapsOption::NoMipmap,
            channel_count,
            neuron_count,
        )
        .unwrap();

        // Set the texture to be filled with random values (use vec of vec for this)
        let mut pixels: Vec<Vec<u8>> = vec![vec![0 as u8; channel_count as usize]; neuron_count as usize];
        for i in 0..neuron_count {
            for j in 0..channel_count {
                //pixels[i as usize][j as usize] = (i * j) as u8;
                pixels[i as usize][j as usize] = rand::random::<u8>();
            }
        }

        texture.write(glium::Rect {
            left: 0,
            bottom: 0,
            width: channel_count,
            height: neuron_count,
        }, pixels);

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
                    uniform sampler2D tex;

                    out vec4 color;

                    void main() {
                        float x = float(gl_FragCoord.x)/float(textureSize(tex, 0).x);
                        float y = float(gl_FragCoord.y)/float(textureSize(tex, 0).y);
                        float t_now = texture(tex, vec2(x, y)).r;
                        
                        color = vec4(t_now, t_now, t_now, 1.0);
                    }
                "
            }
        )
        .unwrap();

        NetworkRenderer {
            running: true,
            display,
            event_loop,
            vertex_buffer,
            index_buffer,
            texture,
            program,
        }
    }

    pub fn render(&self, time: u64) {
        let uniforms = uniform! {
            time: time as u32, // No u64 support. Will cause rendering to be bugged sometimes, but it's not a big deal.
            tex: &self.texture,
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
