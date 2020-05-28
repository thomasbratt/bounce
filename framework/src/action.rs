#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Action {
    None,
    Start,
    Timer,
    Left,
    Right,
    Up,
    Down,
    Quit,
}
