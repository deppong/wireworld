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