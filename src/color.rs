use sdl2::pixels::Color;
pub(crate) struct ColorUtils;
impl ColorUtils{
    pub fn random() -> Color {
        Color::RGB(rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>())
    }
}