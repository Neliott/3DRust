use crate::math_structs::utils::multiply_matrix;
use crate::math_structs::vector2int::Vector2Int;
use crate::rendering::camera::Camera;
use crate::rendering::mesh_renderer::MeshRenderer;
use crate::rendering::screen_drawable::ScreenDrawable;
use crate::rendering::tris::Tris;

pub struct Renderer{
    pub camera: Camera,
    mesh_renderers: Vec<MeshRenderer>,
}

impl Renderer{
    pub fn new(camera: Camera) -> Renderer{
        Renderer{
            camera,
            mesh_renderers: Vec::new(),
        }
    }
    pub fn add_mesh_renderer(&mut self, mesh_renderer: MeshRenderer){
        self.mesh_renderers.push(mesh_renderer);
    }
    pub fn render(&self, mut drawable: &mut (impl ScreenDrawable)){
        drawable.clear();
        for mesh_renderer in self.mesh_renderers.iter(){
            for tris in mesh_renderer.get_tris().iter(){
                self.draw_tris(drawable, tris, mesh_renderer);
            }
        }
        drawable.present();
    }
    fn draw_tris(&self, mut drawable: &mut (impl ScreenDrawable), tris: &Tris, mesh_renderer: &MeshRenderer){
        let screen_width = drawable.size().0 as f32;
        let screen_height = drawable.size().1 as f32;
        let aspect_ratio = screen_width / screen_height;
        let view_projection_matrix = self.camera.get_view_projection_matrix(aspect_ratio);
        let model_matrix = mesh_renderer.get_model_matrix();
        let mvp_matrix = multiply_matrix(view_projection_matrix, model_matrix);
        let tris = tris.to_screen_space(&mvp_matrix);
        self.draw_screen_space_tris(drawable, &tris);
    }

    fn draw_screen_space_tris(&self, mut drawable: &mut (impl ScreenDrawable), tris: &Tris){
        let screen_width = drawable.size().0 as f32;
        let screen_height = drawable.size().1 as f32;
        let vect1 = Vector2Int::new((tris.a.x * screen_width / 2f32 + screen_width / 2f32) as i32, (tris.a.y * screen_height / 2f32 + screen_height / 2f32) as i32);
        let vect2 = Vector2Int::new((tris.b.x * screen_width / 2f32 + screen_width / 2f32) as i32, (tris.b.y * screen_height / 2f32 + screen_height / 2f32) as i32);
        let vect3 = Vector2Int::new((tris.c.x * screen_width / 2f32 + screen_width / 2f32) as i32, (tris.c.y * screen_height / 2f32 + screen_height / 2f32) as i32);
        drawable.draw_line(&vect1, &vect2);
        drawable.draw_line(&vect2, &vect3);
        drawable.draw_line(&vect3, &vect1);
    }
}