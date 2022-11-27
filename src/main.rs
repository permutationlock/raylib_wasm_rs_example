extern crate raylib;

#[cfg(target_family = "wasm")]
extern "C" {
    fn emscripten_set_main_loop(
        loop_fn: extern "C" fn(),
        fps: i32,
        sim_infinite_loop: i32
    );
}

#[cfg(target_family = "wasm")]
#[no_mangle]
pub extern "C" fn on_resize(
    width: i32,
    height: i32
) {
    raylib::set_window_size(width, height);
}

extern "C" fn draw_loop() {
    const TEXT: &[u8; 13] = b"It's working!";

    let width = raylib::get_screen_width();
    let height = raylib::get_screen_height();

    raylib::begin_drawing(); 
    raylib::clear_background(raylib::color::DARKGRAY);

    let twidth = raylib::measure_text(TEXT, 20);
    let theight = 10;
    raylib::draw_text(
        TEXT,
        (width - twidth) / 2,
        (height - theight) / 2,
        20,
        raylib::color::RAYWHITE
    );
    raylib::end_drawing();
}

fn game_loop() {
    #[cfg(target_family = "wasm")]
    unsafe {
        emscripten_set_main_loop(draw_loop, 0, 1);
    }

    #[cfg(target_family = "unix")]
    while !raylib::window_should_close() {
        draw_loop();
    }

    #[cfg(target_family = "windows")]
    while !raylib::window_should_close() {
        draw_loop();
    }
}

fn main() {
    raylib::set_config_flags(raylib::flags::WINDOW_RESIZABLE);
    raylib::init_window(640, 480, "Cross platform Raylib!");
    raylib::set_target_fps(30);
 
    game_loop();
}
