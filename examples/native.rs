use raylib::*;
mod funny_rect;
use funny_rect::*;

fn main() {
    unsafe {
        let mut rect = game_init();
        while !WindowShouldClose() {
            game_frame(&mut rect);
        }
        game_over();
    }
}
