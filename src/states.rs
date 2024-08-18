use crate::game_space::GameBoard;
use std::rc::Rc;

pub struct StateSpace {
    pub game_board: Rc<GameBoard>,
    pub states: Vec<CarState>,
}

impl StateSpace {
    pub fn from(game_board: Rc<GameBoard>) -> StateSpace {
        let mut states = Vec::new();
        for r in 0..game_board.grid.nr {
            for c in 0..game_board.grid.nc {
                for vr in -game_board.max_speed..=game_board.max_speed {
                    for vc in -game_board.max_speed..=game_board.max_speed {
                        let index = Self::raw_index(&game_board, r, c, vr, vc);
                        states.push(CarState { r, c, vr, vc, index });
                    }
                }
            }
        }
        StateSpace { game_board, states }
    }

    pub fn raw_index(game_board: &GameBoard, r: i32, c: i32, vr: i32, vc: i32) -> usize {
        let max_speed = game_board.max_speed;
        let n_speed = 2 * max_speed + 1;
        let mut index = r;
        index = index * game_board.grid.nc + c;
        index = index * n_speed + max_speed + vr;
        index = index * n_speed + max_speed + vc;
        index as usize
    }

    pub fn index(&self, car_state: CarState) -> usize {
        Self::raw_index(&self.game_board, car_state.r, car_state.c, car_state.vr, car_state.vc)
    }

    pub fn get_state(&self, r: i32, c: i32, vr: i32, vc: i32) -> &CarState
    {
        &self.states[Self::raw_index(&self.game_board, r, c, vr, vc)]
    }
}

pub struct CarState {
    pub r: i32,
    pub c: i32,
    pub vr: i32,
    pub vc: i32,
    pub index: usize,
}