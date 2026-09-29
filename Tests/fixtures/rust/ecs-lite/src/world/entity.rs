use crate::components::Component;
use crate::math::Vec2;

pub struct Entity {
    pub pos: Vec2,
    pub vel: Vec2,
    components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new(pos: Vec2) -> Self {
        Entity {
            pos,
            vel: Vec2::ZERO,
            components: Vec::new(),
        }
    }

    /// Attaches a component (generic parameter with a trait bound).
    pub fn with<C: Component + 'static>(mut self, c: C) -> Self {
        self.components.push(Box::new(c));
        self
    }

    pub fn update(&mut self, dt: f32) {
        for c in &mut self.components {
            c.update(dt);
        }
    }

    pub fn component_names(&self) -> Vec<&'static str> {
        self.components.iter().map(|c| c.name()).collect()
    }
}
