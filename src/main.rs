#[macro_use(implement_vertex, program, uniform)]
extern crate glium;

use glium::glutin;
use glium::Surface;
use glium::index::PrimitiveType;

fn main() {
    use glium::DisplayBuild;
    let display = glutin::WindowBuilder::new().with_title("slp-viewer-opengl").build_glium().unwrap();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            pos: [f32; 2],
            color: [f32; 3],
        }

        implement_vertex!(Vertex, pos, color);

        glium::VertexBuffer::new(&display,
            &[
                Vertex { pos: [-1.0, 1.0], color: [1.0, 0.0, 0.0] },
                Vertex { pos: [-1.0, 0.5], color: [0.0, 1.0, 0.0] },
                Vertex { pos: [-0.5, 0.5], color: [0.0, 0.0, 1.0] },
                Vertex { pos: [-0.5, 1.0], color: [0.5, 0.5, 0.5] },
            ]
        ).unwrap()
    };

    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2, 0, 2, 3]).unwrap();

    let program = program!(&display,
        330 => {
            vertex: "#version 330

                layout (location = 0) in vec2 pos;
                layout (location = 1) in vec3 color;
                uniform mat4 matrix;
                out vec3 vColor;

                void main() {
                    gl_Position = vec4(pos, 0.0, 1.0) * matrix;
                    vColor = color;
                }
            ",

            fragment: "#version 330

                in vec3 vColor;
                out vec4 fColor;

                void main() {
                    fColor = vec4(vColor, 1.0);
                }
            ",
        }
    ).unwrap();

    let uniforms = uniform! {
        matrix: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32]
        ]
    };

    'main: loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }
    }
}
