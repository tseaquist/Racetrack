use csv::Error;
use racetrack::display::{display_board, display_solution};
use racetrack::game_space::GameBoard;
use racetrack::opt::Optimization;
use std::collections::HashMap;
use std::env;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() -> Result<(), Error> {
    let game_board = Arc::new(args_to_board(env::args().collect())?);
    display_board(&game_board);

    let opt = Optimization::from(game_board.clone());
    let vals = vec![f64::INFINITY; opt.state_space.states.len()];

    let start = opt.state_space.get_state(game_board.start_r, game_board.start_c, 0, 0);
    let mut start_val = vals[start.index];
    let sync_vals = Arc::from(Mutex::from(vals));

    let now = Instant::now();
    for i in 0..50 {
        opt.value_recursion(sync_vals.clone());
        let sync_vals_2 = sync_vals.lock().unwrap();
        if start_val == sync_vals_2[start.index] && start_val.is_finite() {
            println!("Converged: {start_val}");
            break;
        }
        start_val = sync_vals_2[start.index];
        println!("Step {i}");
    }
    println!("Time to compute: {}", now.elapsed().as_secs_f64());

    // display_vals(&vals, &game_board);
    let temp = sync_vals.lock().unwrap();
    let vals = temp.deref();
    display_solution(&game_board, &opt, vals, start);
    Ok(())
}

fn args_to_board(args: Vec<String>) -> Result<GameBoard, Error> {
    let args = read_args(args);
    let max_speed = args.get("max_speed");
    let max_speed = if max_speed.is_some() { max_speed.unwrap().parse::<i32>().unwrap_or(6) } else { 6 };

    if args.contains_key("circle")
    {
        return Ok(GameBoard::generate_circ_track(40, 40, max_speed));
    }

    let default_path = "./resources/large.csv".to_string();
    let path = args.get("csv_path").unwrap_or(&default_path);
    println!("Path string: {}", path);
    let path = Path::new(path);
    Ok(GameBoard::read_csv_file(path, max_speed)?)
}

fn read_args(args: Vec<String>) -> HashMap<String, String> {
    let mut arg_map = HashMap::<String, String>::new();
    args.iter().for_each(|s| {
        let arg = s.split("=").map(|s| s.to_string()).collect::<Vec<String>>();
        if arg.len() > 1
        {
            arg_map.insert(arg[0].trim().to_string(), arg[1].trim().to_string());
        }
    });
    arg_map
}