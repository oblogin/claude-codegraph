mod sprite;
mod text;

pub use self::sprite::{make_marker, Sprite};
pub use self::text::Text;

/// Anything that can put itself on the render queue.
pub trait Draw {
    fn draw(&self, r: &mut Renderer);

    /// Draw order; lower layers are drawn first.
    fn layer(&self) -> u8 {
        0
    }
}

/// Human-readable description.
pub trait Describe {
    fn describe(&self) -> String;
}

impl<T: Draw> Describe for T {
    fn describe(&self) -> String {
        format!("drawable on layer {}", self.layer())
    }
}

#[derive(Default)]
pub struct Renderer {
    pub commands: Vec<String>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer {
            commands: Vec::new(),
        }
    }

    pub fn push(&mut self, cmd: String) {
        self.commands.push(cmd);
    }

    /// Draws trait objects (dynamic dispatch).
    pub fn draw_all(&mut self, items: &[Box<dyn Draw>]) {
        for item in items {
            item.draw(self);
        }
    }

    /// Draws one item passed as an `impl Trait` argument (static dispatch).
    pub fn draw_one(&mut self, item: &impl Draw) {
        item.draw(self);
    }

    /// Draws a homogeneous slice sorted by layer (generic bound).
    pub fn draw_sorted<T: Draw>(&mut self, items: &mut [T]) {
        items.sort_by_key(|item| item.layer());
        for item in items.iter() {
            item.draw(self);
        }
    }
}

/// Describes every item (bound in a `where` clause).
pub fn summarize<T>(items: &[T]) -> Vec<String>
where
    T: Describe,
{
    let mut out = Vec::new();
    for item in items {
        out.push(item.describe());
    }
    out
}
