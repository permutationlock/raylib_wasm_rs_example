#[allow(unused)] 
pub mod flags {
    #[allow(unused)] 
    pub const WINDOW_RESIZABLE: u32 = 0x00000004;
}

#[allow(unused)] 
pub mod log_level {
    pub const LOG_ALL: i32 = 0;
    pub const LOG_TRACE: i32 = 1;
    pub const LOG_DEBUG: i32 = 2;
    pub const LOG_INFO: i32 = 3;
    pub const LOG_WARNING: i32 = 4;
    pub const LOG_ERROR: i32 = 5;
    pub const LOG_FATAL: i32 = 6;
    pub const LOG_NONE: i32 = 7;
}

#[allow(unused)] 
pub mod keyboard {
    pub const NULL: i32 = 0;
    pub const APOSTROPHE: i32 = 39;
    pub const COMMA: i32 = 44;
    pub const MINUS: i32 = 45;
    pub const PERIOD: i32 = 46;
    pub const SLASH: i32 = 47;
    pub const ZERO: i32 = 48;
    pub const ONE: i32 = 49;
    pub const TWO: i32 = 50;
    pub const THREE: i32 = 51;
    pub const FOUR: i32 = 52;
    pub const FIVE: i32 = 53;
    pub const SIX: i32 = 54;
    pub const SEVEN: i32 = 55;
    pub const EIGHT: i32 = 56;
    pub const NINE: i32 = 57;
    pub const SEMICOLON: i32 = 59;
    pub const EQUAL: i32 = 61;
    pub const A: i32 = 65;
    pub const B: i32 = 66;
    pub const C: i32 = 67;
    pub const D: i32 = 68;
    pub const E: i32 = 69;
    pub const F: i32 = 70;
    pub const G: i32 = 71;
    pub const H: i32 = 72;
    pub const I: i32 = 73;
    pub const J: i32 = 74;
    pub const K: i32 = 75;
    pub const L: i32 = 76;
    pub const M: i32 = 77;
    pub const N: i32 = 78;
    pub const O: i32 = 79;
    pub const P: i32 = 80;
    pub const Q: i32 = 81;
    pub const R: i32 = 82;
    pub const S: i32 = 83;
    pub const T: i32 = 84;
    pub const U: i32 = 85;
    pub const V: i32 = 86;
    pub const W: i32 = 87;
    pub const X: i32 = 88;
    pub const Y: i32 = 89;
    pub const Z: i32 = 90;
    pub const LEFT_BRACKET: i32 = 91;
    pub const BACKSLASH: i32 = 92;
    pub const RIGHT_BRACKET: i32 = 93;
    pub const GRAVE: i32 = 96;

    pub const SPACE: i32 = 32;
    pub const ESCAPE: i32 = 256;
    pub const ENTER: i32 = 257;
    pub const TAB: i32 = 258;
    pub const BACKSPACE: i32 = 259;
    pub const INSERT: i32 = 260;
    pub const DELETE: i32 = 261;
    pub const RIGHT: i32 = 262;
    pub const LEFT: i32 = 263;
    pub const DOWN: i32 = 264;
    pub const UP: i32 = 265;
    pub const PAGE_UP: i32 = 266;
    pub const PAGE_DOWN: i32 = 267;
    pub const HOME: i32 = 268;
    pub const END: i32 = 269;
    pub const CAPS_LOCK: i32 = 280;
    pub const SCROLL_LOCK: i32 = 281;
    pub const NUM_LOCK: i32 = 282;
    pub const PRINT_SCREEN: i32 = 283;
    pub const PAUSE: i32 = 284;
    pub const LEFT_SHIFT: i32 = 340;
    pub const LEFT_CONTROL: i32 = 341;
    pub const LEFT_ALT: i32 = 342;
    pub const RIGHT_SHIFT: i32 = 344;
    pub const RIGHT_CONTROL: i32 = 345;
    pub const RIGHT_ALT: i32 = 346;
    
    pub fn get_char(
        key: i32, shift: bool, caps: bool
    ) -> Option<u8> {
        if key >= A && key <= Z {
            let mut c: u8 = if shift || caps { b'A' }
                else { b'a' };
            Some(c + ((key - A) as u8))
        } else {
            match key {
                APOSTROPHE =>
                    Some(if shift { b'\"' } else { b'\'' }),
                COMMA => Some(if shift { b'<' } else { b',' }),
                MINUS => Some(if shift { b'_' } else { b'-' }),
                PERIOD => Some(if shift { b'>' } else { b'.' }),
                SLASH => Some(if shift { b'?' } else { b'/' }),
                ZERO => Some(if shift { b')' } else { b'0' }),
                ONE => Some(if shift { b'!' } else { b'1' }),
                TWO => Some(if shift { b'@' } else { b'2' }),
                THREE => Some(if shift { b'#' } else { b'3' }),
                FOUR => Some(if shift { b'$' } else { b'4' }),
                FIVE => Some(if shift { b'%' } else { b'5' }),
                SIX => Some(if shift { b'^' } else { b'6' }),
                SEVEN => Some(if shift { b'&' } else { b'7' }),
                EIGHT => Some(if shift { b'*' } else { b'8' }),
                NINE => Some(if shift { b'(' } else { b'9' }),
                SEMICOLON =>
                    Some(if shift { b':' } else { b';' }),
                EQUAL => Some(if shift { b'+' } else { b'=' }),
                LEFT_BRACKET =>
                    Some(if shift { b'{' } else { b'[' }),
                RIGHT_BRACKET =>
                    Some(if shift { b'}' } else { b'}' }),
                BACKSLASH =>
                    Some(if shift { b'|' } else { b'\\' }),
                GRAVE => Some(if shift { b'~' } else { b'`' }),
                SPACE => Some(b' '),
                ENTER => Some(b'\n'),
                TAB => Some(b'\t'),
                _ => None,
            }
        }
    }

    #[link(name = "raylib")]
    extern "C" {
        fn GetKeyPressed() -> i32;    
        fn IsKeyUp(key: i32) -> bool;    
    }

    pub fn get_key_pressed() -> i32 {
        unsafe { GetKeyPressed() }
    }

    pub fn is_key_up(key: i32) -> bool {
        unsafe { IsKeyUp(key) }
    }
}

pub mod text {
}

#[repr(C)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

#[repr(C)]
pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[repr(C)]
pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
    mipmaps: i32,
    format: i32,
}

#[repr(C)]
pub struct Image {
    data: *const libc::c_void,
    width: i32,
    height: i32,
    mipmaps: i32,
    format: i32,
}

#[repr(C)]
pub struct GlyphInfo {
    value: i32,
    offset_x: i32,
    offset_y: i32,
    advance_x: i32,
    image: Image,
}

#[repr(C)]
pub struct Font {
    base_size: i32,
    glyph_count: i32,
    glyph_padding: i32,
    texture: Texture,
    recs: *const Rectangle,
    glyphs: GlyphInfo,
}

#[repr(C)]
pub struct Color {
    x: u8,
    y: u8,
    z: u8,
    w: u8,
}

pub mod color {
    use crate::Color;

    pub const RAYWHITE: Color = Color {
        x: 245, y: 245, z: 245, w: 255
    };
    pub const DARKGRAY: Color = Color {
        x: 80, y: 80, z: 80, w: 255
    };
    pub const MEDGRAY: Color = Color {
        x: 170, y: 170, z: 170, w: 255
    };
}

#[link(name = "raylib")]
extern "C" {
    fn InitWindow(
        width: i32, height: i32,
        title: *const i8
    );
    fn SetTargetFPS(fps: i32);
    fn SetWindowSize(width: i32, height: i32);
    fn CloseWindow();
    fn BeginDrawing();
    fn EndDrawing();
    fn DrawText(
        text: *const i8,
        pos_x: i32,
        pos_y: i32,
        font_size: i32,
        color: Color
    );
    fn MeasureText(text: *const i8, font_size: i32) -> i32;
    fn MeasureTextEx(
        font: Font,
        text: *const i8,
        font_size: f32,
        spacing: f32
    ) -> Vector2;
    fn DrawRectangle(
        pos_x: i32,
        pos_y: i32,
        width: i32,
        height: i32,
        color: Color
    );
    fn ClearBackground(color: Color);
    fn WindowShouldClose() -> bool;
    fn IsWindowResized() -> bool;
    fn SetConfigFlags(flags: u32);
    fn GetScreenWidth() -> i32;
    fn GetScreenHeight() -> i32;
}

pub fn init_window(width: i32, height: i32, title: &str) {
    let c_title = std::ffi::CString::new(title).unwrap();
    unsafe {
        InitWindow(
            width,
            height, 
            c_title.as_ptr()
        );
    }
}

pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps); }
}

pub fn set_window_size(width: i32, height: i32) {
    unsafe { SetWindowSize(width, height); }
}

pub fn close_window() {
    unsafe { CloseWindow(); }
}

pub fn begin_drawing() {
    unsafe { BeginDrawing(); }
}

pub fn end_drawing() {
    unsafe { EndDrawing(); }
}

pub fn draw_text(
    text: &[u8],
    pos_x: i32,
    pos_y: i32,
    font_size: i32,
    color: Color
) {
    let c_text = std::ffi::CString::new(text).unwrap();
    unsafe {
        DrawText(
            c_text.as_ptr(), pos_x, pos_y, font_size, color
        );
    }
}

pub fn measure_text(text: &[u8], font_size: i32) -> i32 { 
    let c_text = std::ffi::CString::new(text).unwrap();
    unsafe { MeasureText(c_text.as_ptr(), font_size) }
}

pub fn measure_text_ex(
    font: Font, text: &[u8], font_size: f32, spacing: f32
) -> Vector2 {
    let c_text = std::ffi::CString::new(text).unwrap();
    unsafe {
        MeasureTextEx(font, c_text.as_ptr(), font_size, spacing)
    }
}

pub fn draw_rectangle(
    pos_x: i32,
    pos_y: i32,
    width: i32,
    height: i32,
    color: Color
) {
    unsafe { DrawRectangle(pos_x, pos_y, width, height, color); }
}

pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color); }
}

pub fn window_should_close() -> bool {
    unsafe { WindowShouldClose() }
}

pub fn is_window_resized() -> bool {
    unsafe { IsWindowResized() }
}

pub fn set_config_flags(flags: u32) {
    unsafe { SetConfigFlags(flags); }
}

pub fn get_screen_width() -> i32 {
    unsafe { GetScreenWidth() }
}

pub fn get_screen_height() -> i32 {
    unsafe { GetScreenHeight() }
}
