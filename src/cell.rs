pub const BOARD_W: u32 = 800;
pub const BOARD_H: u32 = 800;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum State {
    Empty,
    Head,
    Tail,
    Wire
}

impl State {
    pub fn tick(&self, neighbors: usize) -> State {
        match self {
            State::Empty => State::Empty,
            State::Head => State::Tail,
            State::Tail => State::Wire,
            State::Wire => {
                if neighbors == 1 || neighbors == 2 {
                    State::Head
                } else {
                    State::Wire
                }
            },
        }
    }
}

pub struct Board {
    previous: [State; (BOARD_W*BOARD_H) as usize],
    current: [State; (BOARD_W*BOARD_H) as usize]
}

impl Board {
    pub fn get_previous(&self, x: u32, y: u32) -> State {
        self.previous[(x + y * BOARD_W) as usize]
    }

    pub fn get_current(&self, x: u32, y: u32) -> State {
        self.current[(x + y * BOARD_W) as usize]
    }
}