extern crate sdl2;

use std::{time::*, thread};

use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod cell;
use cell::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const RES: u32 = 80;

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

    const BOARD_W: u32 = WIDTH/RES;
    const BOARD_H: u32 = HEIGHT/RES;
    
    let mut old_board = [State::Head; (BOARD_W*BOARD_H) as usize];
    old_board = [
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, 
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, 
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, 
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, 
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, 
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, 
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, 
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, 
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, 
        State::Tail, State::Head, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire, State::Wire
    ];
    let mut new_board = old_board.clone();

    let mut last_time = Instant::now();

    let mut paused = true;

    let dt = 0.01;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), ..} => {
                    break 'running;
                },

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => { paused = !paused; break;},

                _ => {}
            }
        }

        // wireworld loop

        if !paused {
            for y in 0..BOARD_H {
                for x in 0..BOARD_W {
                    let index: usize = (x+y*BOARD_W) as usize;

                    let neighbors = [(x-1, y-1), (x+0, y-1), (x+1, y-1),
                                     (x-1, y+0), (x+0, y+0), (x+1, y+0),
                                     (x-1, y+1), (x+0, y+1), (x+1, y+1),
                    ];

                    let count = neighbors
                        .iter()
                        .filter(|(x, y)| old_board[(x+y*BOARD_W) as usize] == State::Head)
                        .count();
                    new_board[index] = old_board[index].tick(count);
                }
            }
            old_board = new_board;
        }

        // draw loop
        for y in 0..BOARD_H {
            for x in 0..BOARD_W {
                let color = match new_board[(x + y*BOARD_W) as usize] {
                    State::Empty => Color::BLACK,
                    State::Head => Color::CYAN,
                    State::Tail => Color::RED,
                    State::Wire => Color::YELLOW,
                };

                draw_rect(x*RES, y*RES, RES-1, RES-1, color, &mut framedata);
            }
        }

        if Instant::now() - last_time < Duration::from_secs_f32(1.0 * dt) {
            thread::sleep(Duration::from_secs_f32(1.0 * dt) - (Instant::now() - last_time));
            last_time = Instant::now();
        }

        canvas.clear();
        framebuffer.update(None, &framedata, (WIDTH*4) as usize).expect("Texture update");
        canvas.copy(&framebuffer, None, None).expect("oops");
        canvas.present();
    }
    
}
