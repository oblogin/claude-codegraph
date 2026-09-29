use ecs_lite::components::{Health, Velocity};
use ecs_lite::render::{make_marker, summarize, Describe, Sprite, Text};
use ecs_lite::ui::Label;
use ecs_lite::world::entity::Entity;
use ecs_lite::{vec2, Draw, Renderer, World};

fn main() {
    let mut world = World::new();
    let start = vec2!(1, 2);
    let hero = Entity::new(start)
        .with(Health { hp: 10 })
        .with(Velocity(vec2!(3, 4)));
    world.spawn(hero);
    world.update(0.016);

    let tree_pos = vec2!(5, 5);
    let sprite = Sprite::new("tree", tree_pos);
    let mut renderer = Renderer::new();
    sprite.draw(&mut renderer);
    renderer.draw_one(&sprite);

    let marker = make_marker(0.0, 1.0);
    marker.draw(&mut renderer);

    let items: Vec<Box<dyn Draw>> = vec![
        Box::new(sprite.clone()),
        Box::new(Text {
            content: "hello".to_string(),
        }),
    ];
    world.draw(&items);

    let rock = Sprite::new("rock", tree_pos.scale(0.2));
    let mut sprites = vec![sprite.clone(), rock];
    renderer.draw_sorted(&mut sprites);
    for line in summarize(&sprites) {
        println!("{line}");
    }
    println!("{}", sprite.describe());

    let mut label = Label::new("Start");
    label.update("Go");
    println!("{}", label.draw());
    println!("{}", label.describe());
    println!("{} commands, tick {}", renderer.commands.len(), world.tick());
}
