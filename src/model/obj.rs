use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use model::indexed::IndexedModel;
use primitive::vector::Vector4f32;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
pub struct OBJIndex {
    pub vertex_index: i32,
    pub tex_coord_index: i32,
    pub normal_index: i32
}

impl OBJIndex {
    pub fn new() -> OBJIndex {
        OBJIndex {
            vertex_index: 0,
            tex_coord_index: 0,
            normal_index: 0
        }
    }
}

pub struct OBJModel {
    pub positions: Vec<Vector4f32>,
    pub tex_coords: Vec<Vector4f32>,
    pub indices: Vec<OBJIndex>,
    pub tangents: Vec<Vector4f32>,
    pub normals: Vec<Vector4f32>,
    pub has_tex_coords: bool,
    pub has_normals: bool
}

impl OBJModel {
    pub fn new() -> OBJModel {
        OBJModel {
            positions: Vec::new(),
            tex_coords: Vec::new(),
            indices: Vec::new(),
            tangents: Vec::new(),
            normals: Vec::new(),
            has_tex_coords: false,
            has_normals: false
        }
    }

    pub fn init_from_path(&mut self, file_path: String) -> Result<OBJModel, String> {
        let mut positions = Vec::new();
        let mut tex_coords = Vec::new();
        let mut indices = Vec::new();
        let mut tangents = Vec::new();
        let mut normals = Vec::new();
        // let mut has_tex_coords = false;
        // let mut has_normals = false;

        let mut file = try!(File::open(&file_path).map_err(|err| format!("failed to open obj file: {}", err)));
        let mut buffer = String::new();
        try!(file.read_to_string(&mut buffer).map_err(|err| format!("failed to read obj file: {}", err)));

        for line in buffer.lines() {

            let tokens: Vec<&str> = line.split(" ").collect();
            //TODO(dustin): remove empty strings?

            if tokens.len() == 0 || tokens[0] == "#" {
                continue;

            } else if tokens[0] == "v" {

                let x: f32 = try!(tokens[1].parse().map_err(|err| format!("failed to parse token: {}", err)));
                let y: f32 = try!(tokens[2].parse().map_err(|err| format!("failed to parse token: {}", err)));
                let z: f32 = try!(tokens[3].parse().map_err(|err| format!("failed to parse token: {}", err)));

                positions.push( Vector4f32::new(x, y, z, 1_f32));

            } else if tokens[0] == "vt" {

                let x: f32 = try!(tokens[1].parse().map_err(|err| format!("failed to parse token: {}", err)));
                let y: f32 = try!(tokens[2].parse().map_err(|err| format!("failed to parse token: {}", err)));

                tex_coords.push( Vector4f32::new(x, 1_f32 - y, 0_f32, 0_f32));

            } else if tokens[0] == "vn" {

                let x: f32 = try!(tokens[1].parse().map_err(|err| format!("failed to parse token: {}", err)));
                let y: f32 = try!(tokens[2].parse().map_err(|err| format!("failed to parse token: {}", err)));
                let z: f32 = try!(tokens[3].parse().map_err(|err| format!("failed to parse token: {}", err)));

                normals.push( Vector4f32::new(x, y, z, 0_f32));

            } else if tokens[0] == "f" {
                //TODO(dustin): use idiomatic iterators
                for idx in 0..(tokens.len() - 3) {

                    indices.push(try!(self.parse_obj_index(tokens[1 as usize])));
                    indices.push(try!(self.parse_obj_index(tokens[(2 + idx) as usize])));
                    indices.push(try!(self.parse_obj_index(tokens[(3 + idx) as usize])));
                }
            }
        }

        let model = OBJModel {
            positions: positions,
            tex_coords: tex_coords,
            indices: indices,
            tangents: tangents,
            normals: normals,
            has_tex_coords: self.has_tex_coords,
            has_normals: self.has_normals
        };

        Ok(model)
    }

    fn parse_obj_index(&mut self, token: &str) -> Result<OBJIndex, String> {
        let values: Vec<&str> = token.split("/").collect();

        let mut result = OBJIndex::new();
        let vidx: i32 = try!(values[0 as usize].to_string().parse().map_err(|err| format!("failed to parse obj index vertex: {}", err)));
        result.vertex_index = vidx - 1_i32;

        if values.len() > 1 {

            if !values[1].is_empty() {
                self.has_tex_coords = true;
                let tcidx: i32 = try!(values[1 as usize].to_string().parse().map_err(|err| format!("failed to parse obj index tex coord: {}", err)));
                result.tex_coord_index = tcidx - 1_i32;
            }

            if values.len() > 2  {
                self.has_normals = true;
                let nidx: i32 = try!(values[2 as usize].to_string().parse().map_err(|err| format!("failed to parse obj index normal: {}", err)));
                result.normal_index = nidx - 1_i32;
            }

        }

        Ok(result)
    }

    pub fn to_indexed_model(&self) -> IndexedModel {
        let mut result = IndexedModel::new();
        let mut normal_model = IndexedModel::new();

        //TODO(dustin): explicit types
        let mut result_index_map: HashMap<OBJIndex, i32> = HashMap::new();
        let mut normal_index_map: HashMap<i32, i32> = HashMap::new();
        let mut index_map: HashMap<i32, i32> = HashMap::new();

        //TODO(dustin): use idiomatic iterators
        for idx in 0..self.indices.len() {

            let current_index = self.indices[idx as usize];  //NOTE(dustin): maybe as ref not copy see struct

            let current_position = self.positions[current_index.vertex_index as usize]; //NOTE(dustin): maybe as ref not copy see struct
            let current_tex_coord: Vector4f32;
            let current_normal: Vector4f32;

            if self.has_tex_coords {
                current_tex_coord = self.tex_coords[current_index.tex_coord_index as usize]; //NOTE(dustin): maybe as ref not copy see struct
            } else {
                current_tex_coord = Vector4f32::new(0_f32, 0_f32, 0_f32, 0_f32);
            }

            if self.has_normals {
                current_normal = self.normals[current_index.normal_index as usize]; //NOTE(dustin): maybe as ref not copy see struct
            } else {
                current_normal = Vector4f32::new(0_f32, 0_f32, 0_f32, 0_f32);
            }

            //TODO(dustin): fix this crappy unidiomatic code :(
            let mut model_vertex_index = {
                let opt_model_vertex_index = result_index_map.get(&current_index);

                match opt_model_vertex_index {
                    Some(x) => *x,
                    None => -1_i32,
                }
            };
            if model_vertex_index == -1_i32 {

                model_vertex_index = result.positions.len() as i32;
                result_index_map.insert(current_index, model_vertex_index);

                result.positions.push(current_position);
                result.tex_coords.push(current_tex_coord);
                if self.has_normals {
                    result.normals.push(current_normal);
                }
            }

            //TODO(dustin): fix this crappy unidiomatic code :(
            let mut normal_model_index = {
                let opt_normal_model_index = normal_index_map.get(&current_index.vertex_index);

                match opt_normal_model_index {
                    Some(x) => *x,
                    None => -1_i32,
                }
            };
            if normal_model_index == -1_i32 {

                normal_model_index = normal_model.positions.len() as i32;
                normal_index_map.insert(current_index.vertex_index, normal_model_index);

                normal_model.positions.push(current_position);
                normal_model.tex_coords.push(current_tex_coord);
                normal_model.normals.push(current_normal);
                normal_model.tangents.push(Vector4f32::new(0_f32, 0_f32, 0_f32, 0_f32));
            }

            assert!(model_vertex_index != -1_i32);
            assert!(normal_model_index != -1_i32);

            result.indices.push(model_vertex_index);
            normal_model.indices.push(normal_model_index);
            index_map.insert(model_vertex_index, normal_model_index);
        }

        if !self.has_normals {
            normal_model.calc_normals();
            for idx in 0..result.positions.len() as i32 {
                let normal_idx = *index_map.get(&idx).unwrap();
                result.normals.push(normal_model.normals[normal_idx as usize]);
            }
        }

        // normal_model.calc_tangents();
        // for idx in 0..result.positions.len() as i32 {
        //     let tan_idx = *index_map.get(&idx).unwrap();
        //     result.tangents.push(normal_model.tangents[tan_idx as usize]);
        // }

        result
    }
}
