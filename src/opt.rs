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
        let outcome = self.intersecting_cells(state, nvr, nvc);
        outcome.unwrap_or((1.0, self.state_space.get_state(nr, nc, nvr, nvc)))
    }

    fn intersecting_cells<'a>(&'a self, state: &'a CarState, dr: i32, dc: i32) -> Option<(f64, &'a CarState)> {
        //Parameterized vector with t
        //r = r_0 + t * dr
        //c = c_0 + t * dc
        let step_c = if dc > 0 { 1 } else if dc < 0 { -1 } else { 0 };
        let step_r = if dr > 0 { 1 } else if dr < 0 { -1 } else { 0 };
        let t_delta_c = if step_c != 0 { step_c as f64 / dc as f64 } else { f64::INFINITY };
        let t_delta_r = if step_r != 0 { step_r as f64 / dr as f64 } else { f64::INFINITY };

        let mut t_max_c = 0.5 * t_delta_c;
        let mut t_max_r = 0.5 * t_delta_r;

        let mut c = state.c;
        let mut r = state.r;

        let c_final = c + dc;
        let r_final = r + dr;

        let epsilon = 1E-8;
        while c != c_final || r != r_final {
            if t_max_c < t_max_r - epsilon {
                t_max_c += t_delta_c;
                c += step_c;
                let outcome = self.test_and_return_cell(r, c, state);
                if outcome.is_some() {
                    return outcome;
                }
            } else if t_max_c - epsilon > t_max_r {
                t_max_r += t_delta_r;
                r += step_r;
                let outcome = self.test_and_return_cell(r, c, state);
                if outcome.is_some() {
                    return outcome;
                }
            } else { // - epsilon <= t_max_c - t_max_r && t_max_c - t_max_r <= epsilon
                t_max_c += t_delta_c;
                c += step_c;
                // let outcome = self.test_and_return_cell(r, c, state);
                // if outcome.is_some() {
                //     return outcome;
                // }
                t_max_r += t_delta_r;
                r += step_r;
                // let outcome = self.test_and_return_cell(r, c - step_c, state);
                // if outcome.is_some() {
                //     return outcome;
                // }
                let outcome = self.test_and_return_cell(r, c, state);
                if outcome.is_some() {
                    return outcome;
                }
            }
        }
        None
    }

    fn test_and_return_cell<'a>(&'a self, r: i32, c: i32, state: &'a CarState) -> Option<(f64, &'a CarState)> {
        let game_board = self.state_space.game_board.deref();
        if !game_board.is_valid(r, c) {
            return Some((f64::INFINITY, state));
        }
        if game_board.is_goal(r, c) {
            let part = dist(state.r, state.c, r, c);
            let full = dist(state.r, state.c, r, c);
            return Some((part / full, self.state_space.get_state(r, c, 0, 0)));
        }
        None
    }

    pub fn generate_path<'a>(&'a self, vals: &Vec<f64>, start: &'a CarState) -> Vec<(&'a CarState, Option<Action>)> {
        let mut solution = Vec::new();
        let game_board = self.state_space.game_board.deref();
        let mut state = start;
        while !game_board.is_goal(state.r, state.c) {
            let out = self.walk(vals, &state);
            if out.is_some() {
                let out = out.unwrap();
                let action = out.0;
                solution.push((state, Some(action)));
                state = out.1;
            } else {
                eprintln!("Failed to find solution reaching goal. Partial path has been generated.");
                break;
            }
        }
        solution.push((state, None));
        solution
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