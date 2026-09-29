use std::fmt::Debug;

use crate::math::Vec2;

/// Data attached to an entity; `update` has a default no-op body.
pub trait Component: Debug {
    fn name(&self) -> &'static str;

    fn update(&mut self, _dt: f32) {}
}

#[derive(Debug)]
pub struct Health {
    pub hp: i32,
}

impl Component for Health {
    fn name(&self) -> &'static str {
        "health"
    }
}

#[derive(Debug)]
pub struct Velocity(pub Vec2);

impl Component for Velocity {
    fn name(&self) -> &'static str {
        "velocity"
    }

    fn update(&mut self, dt: f32) {
        self.0 = self.0.scale(1.0 - 0.1 * dt);
        if self.0.length() > 10.0 {
            self.0 = self.0.scale(0.5);
        }
    }
}
