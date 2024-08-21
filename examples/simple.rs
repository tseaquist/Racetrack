use csv::Error;
use racetrack::display::{display_board, display_solution};
use racetrack::game_space::GameBoard;
use racetrack::opt::Optimization;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() -> Result<(), Error> {
    let game_board = Arc::new(args_to_board(env::args().collect())?);
    display_board(&game_board);

    let opt = Optimization::from(game_board.clone());
    let mut vals = Vec::with_capacity(opt.state_space.states.len());
    for _ in 0..opt.state_space.states.len() {
        vals.push(Mutex::new(f64::INFINITY));
    }

    let start = opt.state_space.get_state(game_board.start_r, game_board.start_c, 0, 0);
    let mut start_val = *vals[start.index].lock().unwrap();

    let now = Instant::now();
    for i in 0..50 {
        opt.value_recursion(&mut vals);
        if start_val == *vals[start.index].lock().unwrap() && start_val.is_finite() {
            println!("Converged: {start_val}");
            break;
        }
        start_val = *vals[start.index].lock().unwrap();
        println!("Step {i}");
    }
    println!("Time to compute: {}", now.elapsed().as_secs_f64());

    let vals: Vec<f64> = vals.iter().map(|v| *v.lock().unwrap()).collect();
    display_solution(&game_board, &opt, &vals, start);
    Ok(())
}

fn args_to_board(args: Vec<String>) -> Result<GameBoard, Error> {
    let args = read_args(args);
    let max_speed = args.get("max_speed");
    let max_speed = if max_speed.is_some() { max_speed.unwrap().parse::<i32>().unwrap_or(6) } else { 6 };

    if args.contains_key("circle") && args.get("circle").unwrap().parse::<bool>().unwrap_or(false)
    {
        return Ok(GameBoard::generate_circ_track(3, 3, max_speed));
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