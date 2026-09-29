use super::Draw;
use super::Renderer;

pub struct Text {
    pub content: String,
}

impl Text {
    /// Number of characters in the text.
    pub fn length(&self) -> usize {
        self.content.chars().count()
    }
}

impl Draw for Text {
    fn draw(&self, r: &mut Renderer) {
        r.push(format!("text[{}] {}", self.length(), self.content));
    }
}
