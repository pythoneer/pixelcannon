use model::obj::OBJModel;
use primitive::vertex::Vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<i32>
}

impl Mesh {
    pub fn from_path(file_path: String) -> Result<Mesh, String> {
        let model = try!(OBJModel::new().init_from_path(file_path)).to_indexed_model();

        let mut vertices: Vec<Vertex> = Vec::new();
        for idx in 0..model.positions.len() {
            vertices.push(Vertex::new_with_pos_and_texcoords(model.positions[idx as usize], model.tex_coords[idx as usize]));
        }

        let mesh = Mesh{
            vertices: vertices,
            indices: model.indices
        };

        Ok(mesh)
    }
}
