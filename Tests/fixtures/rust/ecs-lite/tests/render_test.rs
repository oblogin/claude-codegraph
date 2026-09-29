use ecs_lite::math::Vec2;
use ecs_lite::render::{Sprite, Text};
use ecs_lite::{vec2, Draw, Renderer};

#[test]
fn draw_all_uses_every_item() {
    let mut r = Renderer::new();
    let origin = vec2!(0, 0);
    let items: Vec<Box<dyn Draw>> = vec![
        Box::new(Sprite::new("a", origin)),
        Box::new(Text {
            content: "b".to_string(),
        }),
    ];
    r.draw_all(&items);
    assert_eq!(r.commands.len(), 2);
}

#[test]
fn sprite_layer_overrides_default() {
    let s = Sprite::new("a", Vec2::new(1.0, 1.0));
    let t = Text {
        content: String::new(),
    };
    assert_eq!(s.layer(), 1);
    assert_eq!(t.layer(), 0);
}

#[test]
fn vector_length() {
    assert_eq!(vec2!(3, 4).length(), 5.0);
}
