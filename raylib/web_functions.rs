use crate::{Rectangle, KeyboardKey, Color};
extern {
    pub fn DrawRectangleRec_(_: *const Rectangle, _: *const Color);
    pub fn DrawRectangle(_: i32, _: i32, _: i32, _: i32, _: Color);
    pub fn SetTargetFPS(_: i32);
    pub fn InitWindow(_: i32, _: i32, _: *const i8);
    pub fn BeginDrawing();
    pub fn ClearBackground_(_: *const Color);
    pub fn GetFrameTime() -> f32;
    pub fn DrawText_(_: *const i8, _: i32, _: i32, _: i32, _: *const Color);
    pub fn EndDrawing();
    pub fn SetExitKey(_: KeyboardKey);
    pub fn CloseWindow();
    pub fn IsKeyDown(_: KeyboardKey) -> bool;
}

// This functions are mandotory because without them all this won't work.
//   You can't just pass a structure into a function and unwrap it in JS via memory buffer,
//   To the "unwrapping data via memory buffer" system to work we've got to create this layer of
//   abscration, maybe i'll find a way to avoid that, but for now, i don't see anyway of doing
//   that, but creating this functions.

pub unsafe fn ClearBackground(color: Color) {
    ClearBackground_(std::ptr::addr_of!(color));
}

pub unsafe fn DrawText(text: *const i8, x: i32, y: i32, size: i32, color: Color) {
    DrawText_(text, x, y, size, std::ptr::addr_of!(color));
}

pub unsafe fn DrawRectangleRec(rec: Rectangle, color: Color) {
    DrawRectangleRec_(std::ptr::addr_of!(rec), std::ptr::addr_of!(color));
}
