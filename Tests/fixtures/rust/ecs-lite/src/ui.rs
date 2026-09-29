/// A text label for menus.
pub struct Label {
    pub text: String,
    pub layer: u8,
}

impl Label {
    pub fn new(text: &str) -> Self {
        Label {
            text: text.to_string(),
            layer: 9,
        }
    }

    /// Returns the label markup.
    pub fn draw(&self) -> String {
        format!("[{}@{}]", self.text, self.layer)
    }

    pub fn describe(&self) -> String {
        format!("label '{}'", self.text)
    }

    pub fn update(&mut self, text: &str) {
        self.text = text.to_string();
    }
}
