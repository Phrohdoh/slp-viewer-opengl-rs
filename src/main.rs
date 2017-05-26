#[macro_use(implement_vertex, program, uniform)]
extern crate glium;

use glium::glutin;
use glium::Surface;
use glium::{IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;
use glium::texture::{SrgbTexture1d, UnsignedTexture2d, UncompressedUintFormat, MipmapsOption,
                     SrgbFormat};
use glium::uniforms::{MinifySamplerFilter, MagnifySamplerFilter};

extern crate clap;
use clap::{App, Arg};

extern crate chariot_palette;
extern crate chariot_slp;

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

    pub fn ortho(left: f32,
                 right: f32,
                 bottom: f32,
                 top: f32,
                 near: f32,
                 far: f32)
                 -> [[f32; 4]; 4] {
        let mut m = Self::identity();
        m.m11 = 2.0 / (right - left);
        m.m22 = 2.0 / (top - bottom);
        m.m33 = -2.0 / (far - near);
        m.m41 = -((right + left) / (right - left));
        m.m42 = -((top + bottom) / (top - bottom));
        m.m43 = -((far + near) / (far - near));

        [[m.m11, m.m12, m.m13, m.m14],
         [m.m21, m.m22, m.m23, m.m24],
         [m.m31, m.m32, m.m33, m.m34],
         [m.m41, m.m42, m.m43, m.m44]]
    }
}

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

#[derive(Copy, Clone)]
struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

implement_vertex!(Vertex, pos, uv);

// dimensions are in world-space
fn create_quad(x: f32, y: f32, width: f32, height: f32) -> [Vertex; 4] {
    [Vertex { pos: [x + 0.0,   y +    0.0], uv: [0.0, 0.0], },
     Vertex { pos: [x + 0.0,   y + height], uv: [0.0, 1.0], },
     Vertex { pos: [x + width, y + height], uv: [1.0, 1.0], },
     Vertex { pos: [x + width, y +    0.0], uv: [1.0, 0.0], }]
}

fn main() {
    let matches = App::new("slp-viewer-opengl")
        .version("0.1.0")
        .author("Taryn Hill <taryn@phrohdoh.com>")
        .about("Render an SLP via OpenGL")
        .arg(Arg::with_name("slp-path")
                 .long("slp-path")
                 .value_name("slp-path")
                 .help("Filepath to the SLP")
                 .required(true)
                 .takes_value(true))
        .arg(Arg::with_name("pal-path")
                 .long("pal-path")
                 .value_name("pal-path")
                 .help("Filepath to the palette (bin) to use")
                 .required(true)
                 .takes_value(true))
        .arg(Arg::with_name("player")
                 .long("player")
                 .value_name("a value ranging from 1 to 8")
                 .help("Player remap index (1..8)")
                 .required(true)
                 .takes_value(true))
        .arg(Arg::with_name("frame-index")
                 .long("frame-index")
                 .value_name("frame-index")
                 .help("A value ranging from 0 to <frame count - 1>")
                 .default_value("0")
                 .takes_value(true))
        .arg(Arg::with_name("debug")
                 .long("debug")
                 .help("In debug mode pixels are rendered as (palette-index, 0, 0)"))
        .get_matches();

    let shape = {
        let slp_path = matches.value_of("slp-path").unwrap();
        let player_idx = {
            let v = matches.value_of("player").unwrap();
            let v = v.parse::<u8>()
                .expect(&format!("Failed to parse {} into an integer value ranging from 1 to 8", v));

            if v > 8 {
                8
            } else if v == 0 {
                1
            } else {
                v
            }
        };

        let mut slp = chariot_slp::SlpFile::read_from_file(slp_path, player_idx)
            .expect(&format!("Failed to read SLP from {}", slp_path));
    
        let frame_idx = {
            let max = (slp.shapes.len() - 1) as u8;
            let v = matches.value_of("frame-index").unwrap();
            let v = v.parse::<u8>()
                .expect(&format!("Failed to parse {} into an integer value ranging from 0 to {}", v, max));

            if v > max {
                max
            } else {
                v
            }
        };

        slp.shapes.swap_remove(frame_idx as usize)
    };

    let pal = {
        let pal_path = matches.value_of("pal-path").unwrap();
        let colors = chariot_palette::read_from_file(pal_path).expect(&format!("Failed to read palette from {}", pal_path));

        let mut rgb = vec![(0u8, 0u8, 0u8); colors.len()];
        for (index, color) in colors.iter().enumerate() {
            rgb[index] = (color.r, color.g, color.b);
        }

        rgb
    };

    use glium::DisplayBuild;
    let display = glutin::WindowBuilder::new()
        .with_title("slp-viewer-opengl")
        .with_dimensions(WIDTH, HEIGHT)
        .build_glium()
        .unwrap();

    let tex_palette =
        SrgbTexture1d::with_format(&display, pal, SrgbFormat::U8U8U8, MipmapsOption::NoMipmap)
            .expect("Failed to create pal_tex");

    let tex_sprite = {
        let sprite_data: Vec<Vec<_>> = shape
            .pixels
            .chunks(shape.header.width as usize)
            .map(|x| x.to_owned())
            .collect();

        UnsignedTexture2d::with_format(&display,
                                       sprite_data,
                                       UncompressedUintFormat::U8,
                                       MipmapsOption::NoMipmap)
                .expect("Failed to create sprite_data_tex")
    };

    let ortho = M::ortho(0f32, WIDTH as f32, HEIGHT as f32, 0f32, 0f32, 1000f32);

    let uniforms = uniform! {
        mOrtho: ortho,
        palette: tex_palette.sampled()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .minify_filter(MinifySamplerFilter::Nearest),
        spriteData: tex_sprite.sampled()
            .magnify_filter(MagnifySamplerFilter::Nearest)
            .minify_filter(MinifySamplerFilter::Nearest),
        isDebug: matches.is_present("debug"),
    };

    let vertex_buffer = VertexBuffer::new(&display,
                                          &create_quad(0.0,
                                                       0.0,
                                                       shape.header.width as f32,
                                                       shape.header.height as f32)).unwrap();

    let index_buffer = IndexBuffer::new(&display,
                                        PrimitiveType::TrianglesList,
                                        &[0u16, 1, 2, 0, 2, 3]).unwrap();

    let program = program!(&display, 330 => { vertex: include_str!("slp.vert"), fragment: include_str!("slp.frag"), }).unwrap();

    let draw_params = glium::DrawParameters {
        blend: glium::Blend::alpha_blending(),
        .. Default::default()
    };

    'main: loop {
        let mut target = display.draw();
        target.clear_color(0.0, 1.0, 0.0, 1.0);
        target
            .draw(&vertex_buffer,
                  &index_buffer,
                  &program,
                  &uniforms,
                  &draw_params)
            .unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glutin::Event::Closed |
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) => break 'main,
                _ => (),
            }
        }
    }
}
