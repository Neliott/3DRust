use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::sys::SDL_Point;
use crate::math_structs::vector2int::Vector2Int;

pub trait ScreenDrawable {
    //fn draw_line(&mut self, a: &Vector2Int, b: &Vector2Int);
    fn draw_pixel(&mut self, a: &Vector2Int, color: Color);
    fn clear(&mut self);
    fn present(&mut self);
    fn size(&self) -> (u32, u32);
}

impl ScreenDrawable for WindowCanvas{
    fn draw_line(&mut self, a: &Vector2Int, b: &Vector2Int){
        self.set_draw_color(Color::RGB(0, 255, 255));
        self.draw_line(SDL_Point{x: a.x, y: a.y}, SDL_Point{x: b.x, y: b.y}).unwrap();
    }

    fn draw_pixel(&mut self, a: &Vector2Int, color: Color){
        self.set_draw_color(color);
        self.draw_point(SDL_Point{x: a.x, y: a.y}).unwrap();
    }

    fn clear(&mut self){
        self.set_draw_color(Color::RGB(0, 0, 0));
        self.clear();
    }
    fn present(&mut self){
        self.present();
    }

    fn size(&self) -> (u32, u32) {
        self.window().drawable_size()
    }
}

