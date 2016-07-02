use memory;

pub trait Drawable {
    fn update(&mut self, buffer: &memory::Memory);
    fn draw(&mut self, buffer: &memory::Memory);
}
