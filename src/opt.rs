use crate::actions::{Action, ActionSpace};
use crate::game_space::{dist, GameBoard};
use crate::states::{CarState, StateSpace};
use rayon::prelude::*;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

pub struct Optimization {
    pub state_space: StateSpace,
    pub action_space: ActionSpace,
}

impl Optimization {
    pub fn from(game_board: Arc<GameBoard>) -> Optimization {
        Optimization {
            state_space: StateSpace::from(game_board.clone()),
            action_space: ActionSpace::from(game_board.clone()),
        }
    }

    pub fn value_recursion(&self, values: &mut Vec<Mutex<f64>>) {
        let game_board = self.state_space.game_board.clone();
        self.state_space.states.par_iter()
            .filter(|s| game_board.is_valid(s.r, s.c))
            // TODO Parallelize over state
            .for_each(|state| {
                if game_board.is_goal(state.r, state.c) {
                    *values[state.index].lock().unwrap() = 0.0;
                } else if !game_board.is_valid(state.r, state.c) {
                    *values[state.index].lock().unwrap() = f64::INFINITY;
                } else {
                    let mut min_val = f64::INFINITY;
                    for action in self.action_space.actions(state) {
                        let incr = self.action_value(&action, state);
                        let future = *values[incr.1.index].lock().unwrap();
                        let val = incr.0 + future;
                        if val < min_val
                        {
                            min_val = val;
                        }
                    }
                    *values[state.index].lock().unwrap() = min_val;
                }
            })
    }

    fn action_value<'a>(&'a self, action: &Action, state: &'a CarState) -> (f64, &'a CarState) {
        let game_board = self.state_space.game_board.deref();
        if game_board.is_goal(state.r, state.c) {
            return (0.0, state);
        }
        let nvr = action.dr as i32 + state.vr;
        let nvc = action.dc as i32 + state.vc;
        let nr = state.r + nvr;
        let nc = state.c + nvc;
        let divs = nvr.abs() + nvc.abs();
        if divs == 0 {
            return (1.0, self.state_space.get_state(nr, nc, nvr, nvc));
        }
        let mut x = state.c as f64;
        let mut y = state.r as f64;
        let dx = nvc as f64 / divs as f64;
        let dy = nvr as f64 / divs as f64;
        //TODO The code below for finding intermediate points can probably be dramatically improved
        for _ in 0..divs {
            x += dx;
            y += dy;
            let cr = x.round() as i32;
            let rr = y.round() as i32;
            if !game_board.is_valid(rr, cr) {
                return (f64::INFINITY, state);
            }
            if game_board.is_goal(rr, cr) {
                let part = dist(state.r, state.c, rr, cr);
                let full = dist(state.r, state.c, nr, nc);
                return (part / full, self.state_space.get_state(rr, cr, nvr, nvc));
            }
        }
        (1.0, self.state_space.get_state(nr, nc, nvr, nvc))
    }

    pub fn walk<'a>(&'a self, values: &Vec<f64>, state: &'a CarState) -> Option<(Action, &'a CarState)> {
        let mut min_val = f64::INFINITY;
        let mut min_action = None;
        let mut min_state = None;
        for action in self.action_space.actions(state) {
            let incr = self.action_value(&action, state);
            let future = values[incr.1.index];
            let val = incr.0 + future;
            if val < min_val
            {
                min_val = val;
                min_action = Some(action);
                min_state = Some(incr.1);
            }
        }
        if min_action.is_some() { Some((min_action.unwrap(), min_state.unwrap())) } else { None }
    }
}