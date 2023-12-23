use macroquad::{prelude::*, main};
use minifemme::*;

#[main("Arcane")]
async fn main() {
    minifemme::start(LevelFilter::Info, LogMode::Pretty);

    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
