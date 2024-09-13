use raylib::prelude::*;

fn main() {

    let screen_width: i32  = 1920;
    let screen_height: i32 = 1080;

    let (mut handler, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Rust Raylib Starter Template")
        .build();

    handler.set_target_fps(60);

    // This is the game loop
    while !handler.window_should_close() {

        // Update state here unless you've delegated the work to another thread

        let mut draw_handler: RaylibDrawHandle<'_> = handler.begin_drawing(&thread);

        // Draw everything here

        draw_handler.clear_background(Color::BLACK);

    }
}
