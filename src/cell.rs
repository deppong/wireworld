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

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Board {
    data: Vec<State>,
    w: usize,
    h: usize,
}

impl Board {
    pub fn new(w: usize, h: usize) -> Self {
        Board {
            w: w,
            h: h,
            data: vec![State::Empty; w*h]
        }
    }

    pub fn get(&self, x: usize, y: usize) -> State {
        if x >= self.w || y >= self.h {
            return State::Empty;
        }
        return self.data[x + y * self.w];
    }

    pub fn set(&mut self, x: usize, y: usize, state: State) {
        if x > self.w || y > self.h {
            panic!("darn"); 
        }
        self.data[x + y * self.w] = state;
    }

    pub fn neighbors(&self, x: usize, y: usize) -> usize {

        let neighbors = [self.get(x-1, y-1),self.get(x-0, y-1),self.get(x+1, y-1),
                                      self.get(x-1, y-0),self.get(x-0, y-0),self.get(x+1, y-0),
                                      self.get(x-1, y+1),self.get(x-0, y+1),self.get(x+1, y+1) ];

        neighbors.iter().filter(|x| **x == State::Head).count()
    }
}