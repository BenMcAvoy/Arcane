use macroquad::{main, prelude::*};
use minifemme::*;
use shipyard::*;

mod ecs;
use ecs::*;

#[main("Arcane")]
async fn main() {
    minifemme::start(LevelFilter::Info, LogMode::Pretty);

    let mut world = World::default();

    world.add_entity(Position {
        x: 128.0,
        y: 128.0
    });

    loop {
        clear_background(BLACK);

        world.run(|positions: View<Position>| {
            for position in positions.iter() {
                draw_rectangle(position.x, position.y, 50., 50., WHITE);
            }
        });

        next_frame().await
    }
}
