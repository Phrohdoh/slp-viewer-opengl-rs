#[macro_use(implement_vertex, program, uniform)]
extern crate glium;

use glium::glutin;
use glium::Surface;
use glium::{IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;
use glium::texture::{SrgbTexture1d, UnsignedTexture2d, UncompressedUintFormat, MipmapsOption, SrgbFormat};
use glium::uniforms::{MinifySamplerFilter, MagnifySamplerFilter};

struct M {
    m11: f32, m12: f32, m13: f32, m14: f32,
    m21: f32, m22: f32, m23: f32, m24: f32,
    m31: f32, m32: f32, m33: f32, m34: f32,
    m41: f32, m42: f32, m43: f32, m44: f32,
}

impl M {
    pub fn identity() -> Self {
        Self {
            m11: 1.0, m12: 0.0, m13: 0.0, m14: 0.0,
            m21: 0.0, m22: 1.0, m23: 0.0, m24: 0.0,
            m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
            m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
        }
    }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
        let mut m = Self::identity();
        m.m11 = 2.0 / (right - left);
        m.m22 = 2.0 / (top - bottom);
        m.m33 = -2.0 / (far - near);
        m.m41 = -((right + left) / (right - left));
        m.m42 = -((top + bottom) / (top - bottom));
        m.m43 = -((far + near) / (far - near));

        [
            [m.m11, m.m12, m.m13, m.m14],
            [m.m21, m.m22, m.m23, m.m24],
            [m.m31, m.m32, m.m33, m.m34],
            [m.m41, m.m42, m.m43, m.m44],
        ]
    }
}

fn main() {
    use glium::DisplayBuild;
    let display = glutin::WindowBuilder::new()
        .with_title("slp-viewer-opengl")
        .with_dimensions(1024, 768)
        .build_glium()
        .unwrap();

    let vertex_buffer = {
        #[derive(Copy, Clone)]
        struct Vertex {
            pos: [f32; 2],
            uv: [f32; 2],
        }

        implement_vertex!(Vertex,
            pos,
            uv
        );

        fn create_quad(width: f32, height: f32) -> [Vertex; 4] {
            [
                Vertex { pos: [  0.0,    0.0], uv: [0.0, 0.0] },
                Vertex { pos: [  0.0, height], uv: [0.0, 1.0] },
                Vertex { pos: [width, height], uv: [1.0, 1.0] },
                Vertex { pos: [width,    0.0], uv: [1.0, 0.0] },
            ]
        }

        VertexBuffer::new(&display, &create_quad(2f32, 2f32)).unwrap()
    };

    let index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2, 0, 2, 3]).unwrap();

    let pal = vec![
        255.0,   0.0,   0.0,
        255.0,   0.0, 255.0,
          0.0,   0.0, 255.0,
        255.0, 255.0, 255.0f32,
    ];

    let pal_tex = SrgbTexture1d::with_format(&display, pal, SrgbFormat::U8U8U8, MipmapsOption::NoMipmap).expect("Failed to create pal_tex");

    let sprite_data = vec![
        vec![ 0, 3u8 ],
        vec![ 2, 1u8 ],
    ];

    let sprite_data_tex = UnsignedTexture2d::with_format(&display, sprite_data, UncompressedUintFormat::U8, MipmapsOption::NoMipmap).expect("Failed to create sprite_data_tex");

    let ortho = M::ortho(0f32, 1024f32, 768f32, 0f32, 0f32, 1000f32);

    let uniforms = uniform! {
        mOrtho: ortho,
        palette: pal_tex.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest),
        spriteData: sprite_data_tex.sampled().magnify_filter(MagnifySamplerFilter::Nearest).minify_filter(MinifySamplerFilter::Nearest),
    };

    let program = program!(&display,
        330 => {
            vertex: include_str!("slp.vert"),
            fragment: include_str!("slp.frag"),
        }
    ).unwrap();

    'main: loop {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed
                | glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape))
                | glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Q))
                    => break 'main,
                _ => ()
            }
        }
    }
}
