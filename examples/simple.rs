use racetrack::display::{display_board, display_solution};
use racetrack::game_space::GameBoard;
use racetrack::opt::Optimization;
use std::rc::Rc;

fn main() {
    let game_board = Rc::new(GameBoard::generate_circ_track(40, 40, 6));

    let opt = Optimization::from(game_board.clone());

    let mut vals = vec![f64::INFINITY; opt.state_space.states.len()];

    let start = opt.state_space.get_state(game_board.start_r, game_board.start_c, 0, 0);
    let mut start_val = vals[start.index];
    for i in 0..50 {
        opt.value_recursion(&mut vals);
        if start_val == vals[start.index] && start_val.is_finite() {
            println!("Converged: {start_val}");
            break;
        }
        start_val = vals[start.index];
        println!("Step {i}");
    }

    // display_vals(&vals, &game_board);
    display_board(&game_board);
    display_solution(&game_board, &opt, &vals, start);
}