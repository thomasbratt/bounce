#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    None,
    Left,
    Right,
    Up,
    Down,
    Quit,
}