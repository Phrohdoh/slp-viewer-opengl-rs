use {chariot_slp, chariot_palette, glium, MipmapsOption, UnsignedTexture2d, UncompressedUintFormat};

implement_vertex!(Vertex, pos, uv);

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pos: [f32; 2],
    uv: [f32; 2],
}

#[derive(Debug)]
pub struct Quad {
    verts: [Vertex; 4],
}

impl Quad {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            // coords are in world-space
            verts: [Vertex { pos: [x + 000.0, y + 0000.0], uv: [0.0, 0.0], },
                    Vertex { pos: [x + 000.0, y + height], uv: [0.0, 1.0], },
                    Vertex { pos: [x + width, y + height], uv: [1.0, 1.0], },
                    Vertex { pos: [x + width, y + 0000.0], uv: [1.0, 0.0], }],
        }
    }

    pub fn into_vertices(self) -> [Vertex; 4] {
        self.verts
    }
}

#[derive(Debug)]
pub struct TexturedGeometry {
    pub tex: UnsignedTexture2d,
    pub geo: Quad,
}

type RgbaU8 = (u8, u8, u8, u8);
fn convert_shape_to_rgba(shape: chariot_slp::SlpLogicalShape, palette: &[(u8, u8, u8)]) -> Vec<RgbaU8> {
    use chariot_slp::DrawCommand::*;

    let mut ret = Vec::new();
    for (palette_index, draw_cmd) in shape.pixels.iter().zip(&shape.commands) {
        match *draw_cmd {
            Skip => ret.push((0, 0, 0, 0)),
            Shadow => ret.push((0, 0, 0, 100)),
            Remap //=> ret.push((0, 0, 0, 254)),
            |
            Color => {
                let (r, g, b) = palette[*palette_index as usize];
                ret.push((r, g, b, 255));
            }
        }
    }

    ret
}

impl TexturedGeometry {
    pub fn from_shape(display_ref: &glium::backend::glutin_backend::GlutinFacade,
                      shape: chariot_slp::SlpLogicalShape,
                      palette: &[(u8, u8, u8)])
                      -> Self {
        let width = shape.header.width;
        let height = shape.header.height;

        let sprite_data: Vec<Vec<_>> = convert_shape_to_rgba(shape, palette)
            .chunks(width as usize)
            .map(|x| x.to_owned())
            .collect();

        Self {
            tex: UnsignedTexture2d::with_format(display_ref,
                                                sprite_data,
                                                UncompressedUintFormat::U8U8U8U8,
                                                MipmapsOption::NoMipmap)
                    .expect("TODO: result"),
            geo: Quad::new(0.0,
                           0.0,
                           width as f32,
                           height as f32),
        }
    }
}