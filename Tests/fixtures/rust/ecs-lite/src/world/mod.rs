pub mod entity;
mod systems;

use self::entity::Entity;
use super::render::{Draw, Renderer};

pub struct World {
    entities: Vec<Entity>,
    pub renderer: Renderer,
    tick: u64,
}

impl World {
    pub fn new() -> Self {
        World {
            entities: Vec::new(),
            renderer: Renderer::new(),
            tick: 0,
        }
    }

    pub fn spawn(&mut self, e: Entity) -> usize {
        self.entities.push(e);
        self.entities.len() - 1
    }

    pub fn update(&mut self, dt: f32) {
        systems::run_physics(self, dt);
        for e in &mut self.entities {
            e.update(dt);
        }
        self.tick += 1;
    }

    pub fn draw(&mut self, items: &[Box<dyn Draw>]) {
        self.renderer.draw_all(items);
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }
}
