extern crate sdl2;

use std::{time::*, thread};

use sdl2::pixels::PixelFormatEnum;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum State {
    Empty,
    Head,
    Tail,
    Wire
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    x: u32,
    y: u32,
    state: State,
}

impl Cell {
    fn tick(&mut self, neighbors: Vec<Cell>) {
        match self.state {
            State::Empty => return,
            State::Head => self.state = State::Tail,
            State::Tail => self.state = State::Wire,
            State::Wire => {
                let electron_heads = neighbors.iter().filter(|x| x.state == State::Head).count();
                if electron_heads == 1 || electron_heads == 2 {
                    self.state = State::Head;
                }
            },
            _ => ()
        }
    }
}

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
    for i in 0..w {
        for j in 0..h {
            put_pixel(x + i, y + j, color, framedata);
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

    const BOARD_W: u32 = 64;
    const BOARD_H: u32 = 64;

    const res: u32 = WIDTH/BOARD_W;
    
    let mut old_board = [State::Empty; (BOARD_W*BOARD_H) as usize];
    let mut new_board = [State::Head; (BOARD_W*BOARD_H) as usize];

    let mut last_time = Instant::now();

    let mut paused = true;

    let dt = 0.0001;

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
            for cell in new_board.iter_mut().filter(|x| **x != State::Empty) {
                match cell {
                    State::Head => *cell = State::Tail,
                    State::Tail => *cell = State::Wire,
                    State::Wire => *cell = State::Wire,
                    _ => ()
                }
            }
        }

        // draw loop
        for i in 0..BOARD_H {
            for j in 0..BOARD_W {
                let color = match new_board[(i + j*BOARD_W) as usize] {
                    State::Empty => Color::BLACK,
                    State::Head => Color::CYAN,
                    State::Tail => Color::RED,
                    State::Wire => Color::YELLOW,
                };

                draw_rect(i*res, j*res, res-1, res-1, color, &mut framedata);
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
