use orbclient::{self, EventIter, Renderer};
use std;

use interpolate::Interpolator;
use model::mesh::Mesh;
use primitive::edge::Edge;
use primitive::matrix::Matrix4f32;
use primitive::vertex::Vertex;
use texture::bitmap::BitmapTexture;

pub struct RenderContext {
    window: orbclient::Window,
}

impl RenderContext {
    pub fn new(width: u32, height: u32, title: &str) -> RenderContext {
        let orb_window = orbclient::Window::new_flags(100, 100, width, height, title, true).unwrap();
        RenderContext{window: orb_window}
    }

    pub fn get_height(&self) -> u32 {
        self.window.height()
    }

    pub fn get_width(&self) -> u32 {
        self.window.width()
    }

    pub fn events(&mut self) -> EventIter {
        self.window.events()
    }

    pub fn clear(&mut self) {
        self.window.set(orbclient::Color { data: 0xFF220CE8});
    }

    pub fn sync(&mut self) {
        self.window.sync();
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh, transform: &Matrix4f32, texture: &BitmapTexture) {
        for idx in (0..mesh.indices.len()).step_by(3) {
            let v1 = &mesh.vertices[mesh.indices[idx as usize] as usize].transform(&transform);
            let v2 = &mesh.vertices[mesh.indices[(idx + 1) as usize] as usize].transform(&transform);
            let v3 = &mesh.vertices[mesh.indices[(idx + 2) as usize] as usize].transform(&transform);

            self.draw_triangle(v1, v2, v3, &texture);
        }
    }

    pub fn draw_triangle(&mut self, v1: &Vertex, v2: &Vertex, v3: &Vertex, texture: &BitmapTexture) {

        //TODO(dustin): optimisation do not calculate/init every time
        let screen_space_transform = Matrix4f32::new().init_sreenspace_transform(self.get_width() as f32 / 2f32, self.get_height() as f32 / 2f32);

        let mut min_vert = v1.transform(&screen_space_transform).perspective_divide();
        let mut mid_vert = v2.transform(&screen_space_transform).perspective_divide();
        let mut max_vert = v3.transform(&screen_space_transform).perspective_divide();

        if min_vert.calc_double_area(&max_vert, &mid_vert) >= 0 {
            return;
        }

        if max_vert.pos.y < mid_vert.pos.y {
            std::mem::swap(&mut mid_vert, &mut max_vert);
        }

        if mid_vert.pos.y < min_vert.pos.y {
            std::mem::swap(&mut mid_vert, &mut min_vert);
        }

        if max_vert.pos.y < mid_vert.pos.y {
            std::mem::swap(&mut max_vert, &mut mid_vert);
        }

        self.scan_triangle(&min_vert, &mid_vert, &max_vert, min_vert.calc_double_area(&max_vert, &mid_vert) >= 0, texture);
    }

    fn scan_triangle(&mut self,  min_vert: &Vertex, mid_vert: &Vertex, max_vert: &Vertex, side: bool, texture: &BitmapTexture) {

        let interpolator = Interpolator::new(min_vert, mid_vert, max_vert);
        let mut top_to_bottom = Edge::new(&interpolator, min_vert, max_vert, 0);
        let mut top_to_middle = Edge::new(&interpolator, min_vert, mid_vert, 0);
        let mut middle_to_bottom = Edge::new(&interpolator, mid_vert, max_vert, 1);

        self.scan_edges(&mut top_to_bottom, &mut top_to_middle, side, texture);
        self.scan_edges(&mut top_to_bottom, &mut middle_to_bottom, side, texture);
    }

    fn scan_edges(&mut self, first: &mut Edge, second: &mut Edge, side: bool, texture: &BitmapTexture) {

        let start_y = second.start_y;
        let end_y = second.end_y;

        let mut left = first;
        let mut right = second;

        if side {
            std::mem::swap(&mut left, &mut right);
        }

        for idx_y in start_y..end_y {
            self.draw_scan_line(&left, &right, idx_y, texture);
            left.step();
            right.step();
        }
    }

    fn draw_scan_line(&mut self, left: &Edge, right: &Edge, idx_y: i32, texture: &BitmapTexture) {

        let min_x = left.pos_x.ceil() as i32;
        let max_x = right.pos_x.ceil()as i32;
        let prestep_x = min_x as f32 - left.pos_x;

        let dist_x = right.pos_x - left.pos_x;
        let tex_coords_step_xx = (right.tex_coords_x - left.tex_coords_x) / dist_x;
        let tex_coords_step_yx = (right.tex_coords_y - left.tex_coords_y) / dist_x;
        let one_over_step_zx = (right.one_over_z - left.one_over_z) / dist_x;

        let mut tex_coords_x = left.tex_coords_x + tex_coords_step_xx * prestep_x;
        let mut tex_coords_y = left.tex_coords_y + tex_coords_step_yx * prestep_x;
        let mut one_over_z = left.one_over_z + one_over_step_zx * prestep_x;

        for idx_x in min_x..max_x {
            let z = 1_f32 / one_over_z;
            let src_x = ((tex_coords_x * z) * (texture.width - 1) as f32 + 0.5_f32) as i32;
            let src_y = ((tex_coords_y * z) * (texture.height - 1) as f32 + 0.5_f32) as i32;

            self.window.pixel(idx_x, idx_y, texture.get_orb_pixel(src_x, src_y));

            one_over_z += one_over_step_zx;
            tex_coords_x += tex_coords_step_xx;
            tex_coords_y += tex_coords_step_yx;
        }
    }
}
