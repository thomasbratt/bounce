#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    None,
    Start,
    Left,
    Right,
    Up,
    Down,
    Quit,
}
