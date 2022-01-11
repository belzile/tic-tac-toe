#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerTurn {
    X,
    O
}

#[derive(PartialEq)]
pub enum CellState {
    Empty,
    X,
    O,
}
