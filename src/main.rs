use sdl2::{
    pixels::Color,
    rect::Rect,
    video::Window,
    image::{
        InitFlag,
        LoadTexture,
    },
    render::{
        Canvas,
        TextureQuery,
    },
    event::{
        Event,
        WindowEvent,
    },
};
use std::{ thread::sleep, time::Duration };

fn main() {
    let sdl_context = sdl2::init().expect("failed context generation");
    let video_subsystem = sdl_context.video().expect("failed video subsystem");
    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();
    
    let mut event_pump = sdl_context.event_pump().expect("failed event pumping");

    let window = video_subsystem.window("test", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .expect("failed window creation");

    let mut center_x = ( window.size().0 / 2 ) as i32;
    let mut center_y = ( window.size().1 / 2 ) as i32;

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .expect("failed window to canvas transform");

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'main },
                Event::Window { win_event, ..} => {
                    match win_event {
                        WindowEvent::Resized(width, height) => {
                            center_x = width / 2;
                            center_y = height / 2;
                            println!("Window size: {}x{}", width, height);
                        },
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(89, 89, 89));
        canvas.clear();

        draw_rect(&mut canvas, center_x, center_y);
        draw_image(&mut canvas, "src/test.png", center_x, center_y);

        canvas.present();
        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_rect(canvas: &mut Canvas<Window>, center_x: i32, center_y:i32) {
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    let size: u32 = 400;
    let x = center_x - ( size / 2 ) as i32;
    let y = center_y - ( size / 2 ) as i32;
    canvas.fill_rect(Rect::new(x, y, size, size)).unwrap();
}

fn draw_image(canvas: &mut Canvas<Window>, src: &str, center_x: i32, center_y:i32) {
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture(src).unwrap();
    let TextureQuery { width, height, .. } = texture.query();
    let scale = 2;
    let w = width * scale;
    let h = height * scale;
    let x = center_x - ( w / 2 ) as i32;
    let y = center_y - ( h / 2 ) as i32;
    canvas.copy(&texture, None, Some(Rect::new(x, y, w, h))).unwrap();
}
