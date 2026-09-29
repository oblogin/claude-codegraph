use super::{Draw, Renderer};
use crate::math::Vec2;

#[derive(Clone, Debug)]
pub struct Sprite {
    pub name: String,
    pub pos: Vec2,
    pub z: u8,
}

impl Sprite {
    pub fn new(name: &str, pos: Vec2) -> Self {
        Sprite {
            name: name.to_string(),
            pos,
            z: 1,
        }
    }
}

impl Draw for Sprite {
    fn draw(&self, r: &mut Renderer) {
        r.push(format!("sprite {} at {:?}", self.name, self.pos));
    }

    fn layer(&self) -> u8 {
        self.z
    }
}

/// Returns an opaque drawable marker (`impl Trait` in return position).
pub fn make_marker(x: f32, y: f32) -> impl Draw {
    let pos = vec2!(x, y);
    Sprite::new("marker", pos)
}
