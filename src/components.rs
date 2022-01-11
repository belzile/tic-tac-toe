use bevy::prelude::Component;
use crate::CellState;

#[derive(Component)]
pub struct TicTacToeCell {
    pub cell_id: u8,
    pub state: CellState,
}
