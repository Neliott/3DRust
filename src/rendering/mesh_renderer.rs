use crate::game::transform::Transform;
use crate::math_structs::matrix4x4::Matrix4x4;
use crate::rendering::mesh::Mesh;
use crate::rendering::tris::Tris;

pub struct MeshRenderer{
    pub transform: Transform,
    mesh: Mesh,
    model_matrix: Matrix4x4
}
impl MeshRenderer{
    pub fn new(transform: Transform, mesh: Mesh) -> MeshRenderer{
        MeshRenderer{
            transform,
            mesh,
            model_matrix: MeshRenderer::identity_matrix()
        }
    }
    pub(crate) fn identity_matrix() -> Matrix4x4{
        let zero : f32 = 0f32;
        let one : f32 = 1f32;
        return [
            [one,zero,zero,zero],
            [zero,one,zero,zero],
            [zero,zero,one,zero],
            [zero,zero,zero,one],
        ]
    }
    pub fn get_model_matrix(&self) -> Matrix4x4{
        let mut matrix : Matrix4x4 = MeshRenderer::identity_matrix();
        matrix[0][3] = self.transform.position.x;
        matrix[1][3] = self.transform.position.y;
        matrix[2][3] = self.transform.position.z;
        matrix
    }

    pub fn get_tris(&self) -> &Vec<Tris>{
        &self.mesh.trices
    }
}