use super::World;
use crate::math::{clamp, Vec2};

const MAX_SPEED: f32 = 5.0;

pub(super) fn run_physics(world: &mut World, dt: f32) {
    let gravity = Vec2::new(0.0, 9.8);
    for e in world.entities.iter_mut() {
        e.vel = e.vel + gravity.scale(dt);
        e.vel.x = clamp(e.vel.x, -MAX_SPEED, MAX_SPEED);
        e.vel.y = e.vel.y.clamp(-MAX_SPEED, MAX_SPEED);
        e.pos = e.pos + e.vel.scale(dt);
    }
}
