use crate::game_space::GameBoard;
use crate::opt::Optimization;
use crate::states::{CarState, StateSpace};

pub fn display_solution(game_board: &GameBoard, opt: &Optimization, vals: &Vec<f64>, start: &CarState) {
    let mut board_display = create_board(game_board);

    let mut state = start;
    let mut i = 0;
    board_display[state.r as usize][state.c as usize] = format!("[{i}]");
    while !game_board.is_goal(state.r, state.c) {
        let out = opt.walk(vals, &state);
        if out.is_some() {
            let out = out.unwrap();
            state = out.1;
            let action = out.0;
            println!("Action {}: A({},{}) -> V({},{})", i, action.dr, action.dc, state.vr, state.vc);
            i += 1;
            board_display[state.r as usize][state.c as usize] = format!("[{}]", i.to_string().chars().last().unwrap());
        }
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