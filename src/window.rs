pub trait Drawable {
    fn update(&mut self);
    fn draw(&mut self);
    fn run(&mut self);
}
