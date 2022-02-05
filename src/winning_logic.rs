use bevy::prelude::*;

use crate::{CellState, GameState, Player, PlayingState, TicTacToeCell};

const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
    // horizontal
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    // vertical
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    // diagonals
    [0, 4, 8],
    [2, 4, 6],
];

pub struct WinningLogicPlugin;

pub struct CheckWinnerEvent;

impl Plugin for WinningLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CheckWinnerEvent>()
            .add_system_set(SystemSet::on_update(PlayingState::Playing).with_system(check_winner));
    }
}

pub fn check_winner(
    mut check_winner_events: EventReader<CheckWinnerEvent>,
    cells_query: Query<&TicTacToeCell>,
    mut update_winner: ResMut<State<GameState>>,
    mut playing_state: ResMut<State<PlayingState>>,
) {
    let events: Vec<&CheckWinnerEvent> = check_winner_events.iter().collect();
    if events.len() == 0 {
        return;
    }

    let mut cells = vec![CellState::Empty; 9];
    for cell in cells_query.iter() {
        cells[cell.cell_id as usize] = cell.state.clone();
    }

    if is_winner(&cells, Player::X) {
        update_winner
            .set(GameState::Won(Player::X))
            .expect("Cannot update winner state");
        playing_state
            .set(PlayingState::NotPlaying)
            .expect("Cannot set playing state");
    } else if is_winner(&cells, Player::O) {
        update_winner
            .set(GameState::Won(Player::O))
            .expect("Cannot update winner state");
        playing_state
            .set(PlayingState::NotPlaying)
            .expect("Cannot set playing state");
    } else if is_draw(&cells) {
        update_winner
            .set(GameState::Draw)
            .expect("Cannot update winner state");
        playing_state
            .set(PlayingState::NotPlaying)
            .expect("Cannot set playing state");
    }
}

fn is_winner(cells: &Vec<CellState>, player: Player) -> bool {
    let state = CellState::Filled(player);
    for winning_combination in WINNING_COMBINATIONS {
        if cells[winning_combination[0]] == state
            && cells[winning_combination[1]] == state
            && cells[winning_combination[2]] == state
        {
            return true;
        }
    }

    return false;
}

fn is_draw(cells: &Vec<CellState>) -> bool {
    !cells.iter().any(|element| *element == CellState::Empty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![CellState::Filled(Player::X), CellState::Filled(Player::O)], true)]
    #[test_case(vec![CellState::Filled(Player::X), CellState::Empty], false)]
    fn test_is_draw(input: Vec<CellState>, expected: bool) {
        assert_eq!(is_draw(&input), expected);
    }

    #[test_case(vec![CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty], Player::X, true)]
    #[test_case(vec![CellState::Empty, CellState::Empty, CellState::Empty, CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Empty], Player::X, true)]
    #[test_case(vec![CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Filled(Player::X), CellState::Filled(Player::X), CellState::Filled(Player::X)], Player::X, true)]
    #[test_case(vec![CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Filled(Player::X), CellState::Empty, CellState::Empty], Player::X, true)]
    #[test_case(vec![CellState::Filled(Player::X), CellState::Filled(Player::O), CellState::Filled(Player::X), CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty, CellState::Empty], Player::X, false)]
    fn test_is_winner(input: Vec<CellState>, player: Player, expected: bool) {
        assert_eq!(is_winner(&input, player), expected);
    }
}
