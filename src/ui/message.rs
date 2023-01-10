#[derive(Debug, Clone)]
pub(crate) enum Message {
    Zoom(Change),
    Move(Direction),
}

#[derive(Debug, Clone)]
pub(crate) enum Change {
    Increase,
    Decrease,
}

#[derive(Debug, Clone)]
pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}
