#[derive(Clone, Copy)]
enum State {
    Empty,
    Head,
    Tail,
    Wire
}

#[derive(Clone, Copy)]
pub struct Cell {
    x: u32,
    y: u32,
    state: State,
}