use fltk::{app, prelude::*, window::Window};

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

fn main() {
    //
    // UI
    //
    const WIDTH: i32 = CHIP8_WIDTH as i32 * 4;
    const HEIGHT: i32 = CHIP8_HEIGHT as i32 * 4;

    let mut window = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_label("Rusty8");

    let mut frame = fltk::frame::Frame::default().size_of(&window);

    window.end();
    window.make_resizable(false);
    window.show();

    let mut frame_buffer = vec![0; (WIDTH * HEIGHT * 4) as usize];

    app::add_idle3(move |_| {
        for (i, pixel) in frame_buffer.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as i16;
            let y = (i / WIDTH as usize) as i16;

            let paint_black = if x < y { true } else { false };

            let rgba = if paint_black {
                [0, 0, 0, 255]
            } else {
                [255, 255, 255, 255]
            };

            pixel.copy_from_slice(&rgba);
        }

        fltk::draw::draw_rgba(&mut frame, &frame_buffer).unwrap();

        window.redraw();
        app::sleep(0.016);
    });

    app::App::default().run().unwrap();
}
