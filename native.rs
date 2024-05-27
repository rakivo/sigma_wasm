mod game;
use game::*;
use raylib::*;

fn main() {
    unsafe {
        game_init();
        while !WindowShouldClose() {
            game_frame();
        }
        game_over();
    }
}
