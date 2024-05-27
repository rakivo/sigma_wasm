extern crate raylib;
use std::ptr::addr_of;
use raylib::KeyboardKey as KEY;

extern {
    fn DrawRectangleRec(_: *const Rectangle, _: *const u8);
    // fn DrawRectangle(_: f32, _: f32, _: f32, _: f32, _: *const u8);
    fn SetTargetFPS(_: u32);
    fn InitWindow(_: usize, _: usize, _: *const u8);
    fn BeginDrawing();
    fn ClearBackground(_: *const u8);
    fn GetFrameTime() -> f32;
    fn DrawText(_: *const u8, _: usize, _: usize, _: usize, _: *const u8);
    fn EndDrawing();
    fn IsKeyDown(_: KEY) -> bool;
}

macro_rules! cstr {
    ($str: expr) => { format!("{}\0", $str).as_ptr() as *const u8 }
}

#[repr(C)]
struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

const SPEED_BOOSTED: f32 = 1550.0;
const SPEED_DEFAULT: f32 = 850.0;

const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;

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
}

#[no_mangle]
pub unsafe fn game_frame() {
    handle_keys();
    BeginDrawing();
        ClearBackground(cstr!("red"));
        DrawText(cstr!("omg tf is happening"), 250, 500, 50, cstr!("white"));
        DrawRectangleRec(addr_of!(RECT), cstr!("black"));
    EndDrawing();
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
