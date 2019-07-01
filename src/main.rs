use rand::Rng;
use sdl2::event::Event;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, TextureCreator};

const FIRE_WIDTH: u32 = 320;
const FIRE_HEIGHT: u32 = 168;
const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 600;

fn main() {
    // Every 3 values represent R, G, B.
    // The values 0->32 are used to select which grouping of 3 we want to use.
    // E.g. 0 = 0x070707, 6 = 0x671F07, 36 = 0xFFFFFF
    // Colours at the start are colder and get hotter the deeper we go into the palette.
    let color_palette = [
        0x07, 0x07, 0x07, 0x1F, 0x07, 0x07, 0x2F, 0x0F, 0x07, 0x47, 0x0F, 0x07, 0x57, 0x17, 0x07,
        0x67, 0x1F, 0x07, 0x77, 0x1F, 0x07, 0x8F, 0x27, 0x07, 0x9F, 0x2F, 0x07, 0xAF, 0x3F, 0x07,
        0xBF, 0x47, 0x07, 0xC7, 0x47, 0x07, 0xDF, 0x4F, 0x07, 0xDF, 0x57, 0x07, 0xDF, 0x57, 0x07,
        0xD7, 0x5F, 0x07, 0xD7, 0x5F, 0x07, 0xD7, 0x67, 0x0F, 0xCF, 0x6F, 0x0F, 0xCF, 0x77, 0x0F,
        0xCF, 0x7F, 0x0F, 0xCF, 0x87, 0x17, 0xC7, 0x87, 0x17, 0xC7, 0x8F, 0x17, 0xC7, 0x97, 0x1F,
        0xBF, 0x9F, 0x1F, 0xBF, 0x9F, 0x1F, 0xBF, 0xA7, 0x27, 0xBF, 0xA7, 0x27, 0xBF, 0xAF, 0x2F,
        0xB7, 0xAF, 0x2F, 0xB7, 0xB7, 0x2F, 0xB7, 0xB7, 0x37, 0xCF, 0xCF, 0x6F, 0xDF, 0xDF, 0x9F,
        0xEF, 0xEF, 0xC7, 0xFF, 0xFF, 0xFF,
    ];
    // Create buffer and set all pixels to black.
    let mut pixel_buffer = vec![0; (FIRE_WIDTH * FIRE_HEIGHT) as usize];

    // Set the bottom line to white ("hot").
    for i in 0..FIRE_WIDTH {
        let bottom_x_y = ((FIRE_HEIGHT - 1) * FIRE_WIDTH + i) as usize;
        pixel_buffer[bottom_x_y] = 36;
    }

    // Set Up SDL Window & Canvas
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Rust Doom Fire FX", CANVAS_WIDTH, CANVAS_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .target_texture()
        .present_vsync()
        .build()
        .unwrap();

    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    // RGBA8888 splits each pixel into four 8 bit sections taking a total of 4 bytes.
    // This is how we'll set Red, Green Blue and Alpha.
    let mut fire_texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA8888, FIRE_WIDTH, FIRE_HEIGHT)
        .map_err(|e| e.to_string())
        .unwrap();

    // Start with a blank slate and then present it for viewing.
    canvas.clear();
    canvas.set_draw_color(Color::RGBA(0x07, 0x07, 0x07, 255));
    canvas.present();

    // Spooky
    let mut y_scrolling = 600;
    let image_texture_creator = canvas.texture_creator();

    // Ferris Logo:
    let logo = image_texture_creator
        .load_texture("./res/skellebones.png")
        .unwrap();

    let mut fire_direction = 1;

    // This gives us access to keyboard events.
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Left => fire_direction -= 1,
                    Keycode::Right => fire_direction += 1,
                    Keycode::R => fire_direction = 1,
                    _ => {}
                },
                _ => {}
            }
        }

        // Write state of pixel buffer into fire texture.
        fire_texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                calculate_fire(&mut pixel_buffer, fire_direction);

                for (idx, pixel_cursor) in pixel_buffer.iter().enumerate() {
                    let start = (*pixel_cursor * 3) as usize;
                    let end = start + 3;

                    match &color_palette[start..end] {
                        [red, green, blue] => {
                            let mut alpha = 255;

                            // Dark pixles are transparent.
                            if [*red, *green, *blue].iter().all(|color| color <= &0x07) {
                                alpha = 0;
                            }

                            let offset = idx * 4;
                            buffer[offset] = alpha as u8;
                            buffer[offset + 1] = *blue;
                            buffer[offset + 2] = *green;
                            buffer[offset + 3] = *red;
                        }
                        _ => (),
                    }
                }
            })
            .unwrap();

        &fire_texture.set_blend_mode(BlendMode::Blend);

        if y_scrolling != 70 {
            y_scrolling -= 2;
        }

        let rect = Rect::new(0, 0, CANVAS_WIDTH, CANVAS_HEIGHT);
        let spooky_rect = Rect::new(40, y_scrolling, CANVAS_WIDTH - 75, 450);

        canvas.copy(&logo, None, Some(spooky_rect)).unwrap();
        canvas.copy(&fire_texture, None, Some(rect)).unwrap();
        canvas.present();
    }
}

pub fn calculate_fire(pixel_buffer: &mut [u8], fire_direction: u32) {
    for x in 0..FIRE_WIDTH {
        for y in 1..FIRE_HEIGHT {
            let pixel_cursor = y * FIRE_WIDTH + x;
            spread_fire(pixel_cursor, pixel_buffer, fire_direction);
        }
    }
}

pub fn spread_fire(pixel_cursor: u32, pixel_buffer: &mut [u8], fire_direction: u32) {
    let pixel = pixel_buffer[pixel_cursor as usize];

    if pixel == 0 {
        // Black is too cold to affect anything.
        let index = (pixel_cursor - FIRE_WIDTH) as usize;
        pixel_buffer[index] = 0;
    } else {
        // Random value [0, 1, 2];
        let mut rng = rand::thread_rng();
        let random_index = (rng.gen::<f64>() * 3.0).round() as u8 & 3;

        // Distance affects how the fire behaves. E.g. blowing left, right.
        let distance = pixel_cursor - (random_index as u32) + fire_direction;
        let new_index = (distance - FIRE_WIDTH) as usize;
        //
        // Select a similar colour for the near pixel.
        pixel_buffer[new_index] = pixel - (random_index & 1);
    }
}
