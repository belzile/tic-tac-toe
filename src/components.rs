use crate::CellState;
use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct TicTacToeCell {
    pub cell_id: u8,
    pub state: CellState,
}
