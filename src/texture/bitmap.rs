use orbclient;
use orbimage::Image;

//NOTE(dustin): format ARGB
pub struct BitmapTexture {
    pub width: i32,
    pub height: i32,
    pub data: Vec<u8>
}

//TODO:(dustin) use orbclient color format, avoid expensive conversation
impl BitmapTexture {
    pub fn new(_width: i32, _height: i32) -> BitmapTexture {
        BitmapTexture {
            width: _width,
            height: _height,
            data: vec![0_u8; (_width * _height * 4) as usize]
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, a: u8, r: u8, g: u8, b: u8) {
        let idx = ((x + y * self.width) * 4) as usize;
        self.data[idx    ] = a;
        self.data[idx + 1] = r;
        self.data[idx + 2] = g;
        self.data[idx + 3] = b;
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> (u8, u8, u8, u8) {
        let tex_idx = ((x + y * self.width) * 4) as usize;

        let a = self.data[tex_idx];
        let r = self.data[tex_idx + 1];
        let g = self.data[tex_idx + 2];
        let b = self.data[tex_idx + 3];

        (a, r, g, b)
    }

    pub fn get_orb_pixel(&self, x: i32, y: i32) -> orbclient::Color {
        let (a, r, g, b) = self.get_pixel(x, y);
        let color = ((a as u32) << 24) + ((r as u32) << 16) + ((g as u32) << 8) + b as u32;

        orbclient::Color { data: color }
    }

    pub fn from_orbimage(image: &Image) -> BitmapTexture {
        let mut texture = BitmapTexture::new(image.width() as i32, image.height() as i32);

        for x in 0..texture.width {
            for y in 0..texture.height {

                let col_idx = (x + y * image.width() as i32) as usize;
                let orb_color = image.data()[col_idx];

                let r = (orb_color.data >> 16) as u8;
                let g = (orb_color.data >> 8) as u8;
                let b = orb_color.data as u8;
                texture.set_pixel(x, y, 255, r, g, b);
            }
        }

        texture
    }
    // pub fn copy_pixel_from_texture(&mut self, dest_x: i32, dest_y: i32, src_x: i32, src_y: i32, texture: &BitmapTexture) {
    //
    //     let dest_idx = ((dest_x + dest_y * self.width) * 4) as usize;
    //     let src_idx = ((src_x + src_y * texture.width) * 4) as usize;
    //
    //     self.data[dest_idx    ] = texture.data[src_idx];
    //     self.data[dest_idx + 1] = texture.data[src_idx + 1];
    //     self.data[dest_idx + 2] = texture.data[src_idx + 2];
    //     self.data[dest_idx + 3] = texture.data[src_idx + 3];
    // }

    // pub fn copy_to_byte_array(& self, &mut)
}
