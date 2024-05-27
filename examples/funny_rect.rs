extern crate raylib;
use raylib::{*, KeyboardKey as KEY};

const SPEED_BOOSTED: f32 = 1550.0;
const SPEED_DEFAULT: f32 = 850.0;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;

static mut SPEED: f32 = 850.0;

#[no_mangle]
pub unsafe fn game_init() -> Rectangle {
    SetTargetFPS(144);
    InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, cstr!("Game"));
    SetExitKey(KEY::Q);

    Rectangle {
        x: (WINDOW_WIDTH as f32 - 100.0)/2.0,
        y: (WINDOW_HEIGHT as f32 - 100.0)/2.0,
        width: 100.0,
        height: 100.0
    }
}

#[no_mangle]
pub unsafe fn game_frame(state: &mut Rectangle) {
    handle_keys(state);
    BeginDrawing();
        ClearBackground(RED);
        DrawFPS(0, 0);
        DrawText(cstr!("omg tf is happening"), 250, 500, 50, WHITE);
        DrawRectangleRec(*state, WHITE);
    EndDrawing();
}

#[no_mangle]
pub unsafe fn game_over() {
    CloseWindow();
}

unsafe fn handle_keys(state: &mut Rectangle) {
    let dt = GetFrameTime();
    if IsKeyDown(KEY::Space) { SPEED = SPEED_BOOSTED; }
    if !IsKeyDown(KEY::Space) { SPEED = SPEED_DEFAULT; }
    if IsKeyDown(KEY::W) { state.y -= dt*SPEED; }
    if IsKeyDown(KEY::A) { state.x -= dt*SPEED; }
    if IsKeyDown(KEY::S) { state.y += dt*SPEED; }
    if IsKeyDown(KEY::D) { state.x += dt*SPEED; }
}
