use std::fmt::Display;

pub trait Drawable {
    fn update(&mut self);
    fn pause(&mut self);
    fn draw(&mut self);
}

pub trait Window: Drawable + Display {}

impl <T: Drawable + Display> Window for T {}
