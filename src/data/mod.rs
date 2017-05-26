use {chariot_slp, glium, MipmapsOption, UnsignedTexture2d, UncompressedUintFormat};

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

impl TexturedGeometry {
    pub fn from_shape(display_ref: &glium::backend::glutin_backend::GlutinFacade,
                      shape: chariot_slp::SlpLogicalShape)
                      -> Self {
        let sprite_data: Vec<Vec<_>> = shape
            .pixels
            .chunks(shape.header.width as usize)
            .map(|x| x.to_owned())
            .collect();

        Self {
            tex: UnsignedTexture2d::with_format(display_ref,
                                                sprite_data,
                                                UncompressedUintFormat::U8,
                                                MipmapsOption::NoMipmap)
                    .expect("TODO: result"),
            geo: Quad::new(0.0,
                           0.0,
                           shape.header.width as f32,
                           shape.header.height as f32),
        }
    }
}