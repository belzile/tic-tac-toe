#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Player {
    X,
    O
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerTurn {
    X,
    O,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CellState {
    Empty,
    Filled(Player),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WinnerState {
    XWon,
    OWon,
    Draw,
    GameOngoing,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    NotPlaying,
    Local,
}
