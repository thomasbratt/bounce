#[derive(Debug, Clone, PartialEq)]
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
