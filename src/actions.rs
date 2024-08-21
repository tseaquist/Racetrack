use crate::game_space::GameBoard;
use crate::states::CarState;
use std::sync::Arc;

pub struct ActionSpace {
    game_board: Arc<GameBoard>,
}

pub struct Action {
    pub dr: i8,
    pub dc: i8,
}

impl Action {
    fn new(dr: i8, dc: i8) -> Action {
        Action { dr, dc }
    }
}

impl ActionSpace {
    pub fn from(game_board: Arc<GameBoard>) -> ActionSpace {
        ActionSpace { game_board }
    }

    pub fn actions(&self, state: &CarState) -> Vec<Action> {
        let mut actions = Vec::with_capacity(9);
        let rows = self.game_board.grid.nr;
        let cols = self.game_board.grid.nc;
        for dr in -1..=1 {
            for dc in -1..=1 {
                let nvr = dr + state.vr;
                let nvc = dc + state.vc;
                let nr = state.r + nvr;
                let nc = state.c + nvc;
                if nvr.abs() <= self.game_board.max_speed &&
                    nvc.abs() <= self.game_board.max_speed &&
                    0 <= nr && nr < rows &&
                    0 <= nc && nc < cols
                {
                    actions.push(Action::new(dr as i8, dc as i8));
                }
            }
        }
        actions
    }
}