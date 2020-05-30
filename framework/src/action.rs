#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Action {
    Start,
    Timer,
    Left,
    Right,
    Up,
    Down,
    Quit,
}
