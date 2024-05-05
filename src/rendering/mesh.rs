use crate::math_structs::vector3::Vector3;
use crate::math_structs::vector3int::Vector3Int;
use crate::rendering::tris::Tris;

pub struct Mesh{
    pub trices: Vec<Tris>
}

impl Mesh {
    pub fn new(vertices: Vec<Vector3>, indices: Vec<Vector3Int>) -> Mesh{
        Mesh{
            trices: Mesh::compute_trices(indices, vertices)
        }
    }
    pub fn compute_trices(indices: Vec<Vector3Int>, vertices: Vec<Vector3>) -> Vec<Tris>{
        indices.iter().map(|indices| Tris::new(
            vertices[indices.x as usize],
            vertices[indices.y as usize],
            vertices[indices.z as usize]
        )).collect()
    }
}