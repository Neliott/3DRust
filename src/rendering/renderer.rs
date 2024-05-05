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
        //Only call draw the pixel
        let start_x = vect1.x.min(vect2.x).min(vect3.x);
        let end_x = vect1.x.max(vect2.x).max(vect3.x);
        let start_y = vect1.y.min(vect2.y).min(vect3.y);
        let end_y = vect1.y.max(vect2.y).max(vect3.y);
        for x in start_x..end_x{
            for y in start_y..end_y{
                let point = Vector2Int::new(x, y);
                if Renderer::point_in_triangle(&point, &vect1, &vect2, &vect3){
                    drawable.draw_pixel(&point, tris.color);
                }
            }
        }
    }

    fn point_in_triangle(point: &Vector2Int, vect1: &Vector2Int, vect2: &Vector2Int, vect3: &Vector2Int) -> bool{
        let b1 = Renderer::sign(point, vect1, vect2) < 0.0;
        let b2 = Renderer::sign(point, vect2, vect3) < 0.0;
        let b3 = Renderer::sign(point, vect3, vect1) < 0.0;
        return ((b1 == b2) && (b2 == b3));
    }

    fn sign(p1: &Vector2Int, p2: &Vector2Int, p3: &Vector2Int) -> f32{
        return (p1.x - p3.x) as f32 * (p2.y - p3.y) as f32 - (p2.x - p3.x) as f32 * (p1.y - p3.y) as f32;
    }
}