use crate::actions::Action;
use crate::game_space::GameBoard;
use crate::states::{CarState, StateSpace};

pub fn display_solution(game_board: &GameBoard, path: &Vec<(&CarState, Option<Action>)>) {
    let mut board_display = create_board(game_board);
    for i in 0..path.len() {
        let out = path.get(i);
        let out = out.unwrap();
        let state = out.0;
        let action_opt = &out.1;
        if let Some(action) = action_opt {
            println!("Action {}: S({},{},{},{}) + A({},{})", i, state.r, state.c, state.vr, state.vc, action.dr, action.dc);
        } else {
            println!("Goal Reached: P({},{}))", state.r, state.c);
        }
        board_display[state.r as usize][state.c as usize] = format!("[{}]", i.to_string().chars().last().unwrap());
    }
    display_from_board(&board_display);
}

fn create_board(game_board: &GameBoard) -> Vec<Vec<String>> {
    let nr = game_board.grid.nr;
    let nc = game_board.grid.nc;
    let mut board_display = vec![vec!["[ ]".to_string(); nc as usize]; nr as usize];
    for r in 0..nr {
        for c in 0..nc {
            if !game_board.is_valid(r, c) {
                board_display[r as usize][c as usize] = "[X]".to_string();
            } else if game_board.is_goal(r, c) {
                board_display[r as usize][c as usize] = "[!]".to_string();
            } else if game_board.start_r == r && game_board.start_c == c {
                board_display[r as usize][c as usize] = "[S]".to_string();
            }
        }
    }
    board_display
}

pub fn display_board(game_board: &GameBoard) {
    let board_display = create_board(game_board);
    display_from_board(&board_display);
}
fn display_from_board(board: &Vec<Vec<String>>) {
    for r in (0..board.len()).rev() {
        for c in 0..board[0].len() {
            print!("{}", board[r][c]);
        }
        println!();
    }
}

pub fn display_vals(vals: &Vec<f64>, game_board: &GameBoard) {
    let max_speed = game_board.max_speed;
    let nr = game_board.grid.nr;
    let nc = game_board.grid.nc;
    for vr in -max_speed..=max_speed {
        for vc in -max_speed..=max_speed {
            println!("Velocity: ({},{})", vr, vc);
            for r in (0..nr).rev() {
                for c in 0..nc {
                    print!("{:.3},\t", vals[StateSpace::raw_index(game_board, r, c, vr, vc)])
                }
                println!();
            }
        }
    }
}