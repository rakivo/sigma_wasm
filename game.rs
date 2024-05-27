extern crate raylib;
use raylib::{*, KeyboardKey as KEY};

macro_rules! cstr {
    ($str: expr) => { format!("{}\0", $str).as_ptr() as *const i8 }
}

const SPEED_BOOSTED: f32 = 1550.0;
const SPEED_DEFAULT: f32 = 850.0;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

static mut SPEED: f32 = 850.0;

static mut RECT: Rectangle = Rectangle {
    x: (WINDOW_WIDTH as f32 - 100.0)/2.0,
    y: (WINDOW_HEIGHT as f32 - 100.0)/2.0,
    width: 100.0,
    height: 100.0
};

#[no_mangle]
pub unsafe fn game_init() {
    SetTargetFPS(144);
    InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, cstr!("Game"));
    SetExitKey(KEY::Q);
}

#[no_mangle]
pub unsafe fn game_frame() {
    handle_keys();
    BeginDrawing();
        ClearBackground(RED);
        DrawText(cstr!("omg tf is happening"), 250, 500, 50, WHITE);
        DrawRectangleRec(RECT, WHITE);
    EndDrawing();
}

#[no_mangle]
pub unsafe fn game_over() {
    CloseWindow();
}

unsafe fn handle_keys() {
    let dt = GetFrameTime();
    if IsKeyDown(KEY::Space) { SPEED = SPEED_BOOSTED; }
    else if !IsKeyDown(KEY::Space) { SPEED = SPEED_DEFAULT; }
    if IsKeyDown(KEY::W) { RECT.y -= dt*SPEED; }
    if IsKeyDown(KEY::A) { RECT.x -= dt*SPEED; }
    if IsKeyDown(KEY::S) { RECT.y += dt*SPEED; }
    if IsKeyDown(KEY::D) { RECT.x += dt*SPEED; }
}

#[allow(unused)]
#[cfg(feature = "native")]
fn main() {
    unsafe {
        game_init();
        while !WindowShouldClose() {
            game_frame();
        }
        game_over();
    }
}
