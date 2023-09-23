extern crate sdl2;

use std::time::*;

use sdl2::mouse::MouseButton;
use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod cell;
use cell::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const RES: u32 = 20;

/*
 *  Drawing functions
*/
fn put_pixel(x: u32, y: u32, color: Color, framedata: &mut Vec<u8>) {
    framedata[((x + y * WIDTH)*4 + 0) as usize] = color.b;
    framedata[((x + y * WIDTH)*4 + 1) as usize] = color.g;
    framedata[((x + y * WIDTH)*4 + 2) as usize] = color.r;
    framedata[((x + y * WIDTH)*4 + 3) as usize] = color.a;
}

fn draw_rect(x: u32, y: u32, w: u32, h: u32, color: Color, framedata: &mut Vec<u8>) {
    for i in 0..h {
        for j in 0..w {
            put_pixel(x + j, y + i, color, framedata);
        }
    }
}

fn grid_from_mouse(x: i32, y: i32) -> (usize, usize) {
    // annoying casting, if it works it works though, right?
    let x = x as usize;
    let y = y as usize;
    let res = RES as usize;

    ( 
        if x > res { (x - res) / res + 1} else { 0 },
        if y > res { (y - res) / res + 1} else { 0 }
    )
}

// --------------------

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Wireworld", WIDTH, HEIGHT).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut framebuffer = texture_creator.create_texture_streaming(Some(PixelFormatEnum::ARGB8888), WIDTH, HEIGHT).unwrap();
    let mut framedata: Vec<u8> = vec![0; ((WIDTH*HEIGHT)*4) as usize];

    let mut event_pump = sdl_context.event_pump().unwrap();

    const BOARD_W: usize = (WIDTH/RES) as usize;
    const BOARD_H: usize = (HEIGHT/RES) as usize;

    let mut next = Board::new(BOARD_W, BOARD_H);
    let mut current = Board::new(BOARD_W, BOARD_H);
    current.set(BOARD_W/2, BOARD_H/2, State::Head);
    next.set(BOARD_W/2, BOARD_H/2, State::Head);
    
    let mut last_time = Instant::now();

    let mut paused = true;
    let mut drawing = false;

    let dt = 0.1;

    let mut pen: State = State::Wire;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },

                Event::KeyDown { keycode, .. } => { 
                    match keycode {
                        Some(Keycode::Space) => paused = !paused,
                        Some(Keycode::Num1) => pen = State::Empty,
                        Some(Keycode::Num2) => pen = State::Wire,
                        Some(Keycode::Num3) => pen = State::Head,
                        Some(Keycode::Num4) => pen = State::Tail,
                        _ => ()
                    }
                },

                Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, ..} => { 
                    drawing = true; 
                    let a = grid_from_mouse(x, y);
                    current.set(a.0, a.1, pen);
                }

                Event::MouseButtonUp { mouse_btn: MouseButton::Left, ..} => { drawing = false; }

                Event::MouseMotion {x, y, ..} => {
                    if drawing {
                        let a = grid_from_mouse(x, y);
                        current.set(a.0, a.1, pen);
                    }
                }

                _ => {}
            }
        }

        // wireworld loop

        if !paused && Instant::now() - last_time > Duration::from_secs_f32(1.0 * dt){
            for y in 0..BOARD_H {
                for x in 0..BOARD_W {
                    next.set(x,y, current.get(x, y).tick(current.neighbors(x, y)));
                }
            }
            current = next.clone();
            
            last_time = Instant::now();
        }

        // draw loop
        for y in 0..BOARD_H {
            for x in 0..BOARD_W {
                let color = match current.get(x, y) {
                    State::Empty => Color::BLACK,
                    State::Head => Color::CYAN,
                    State::Tail => Color::RED,
                    State::Wire => Color::YELLOW,
                };

                draw_rect(x as u32*RES, y as u32*RES, RES-1, RES-1, color, &mut framedata);
            }
        }

        canvas.clear();
        framebuffer.update(None, &framedata, (WIDTH*4) as usize).expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }
    
}
