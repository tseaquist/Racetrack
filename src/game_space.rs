use crate::predicates::{GridPredicate, Mask, SingletonPredicate};
use std::fs::File;
use std::io::Error;
use std::path::Path;

pub struct GameBoard {
    pub grid: Grid,
    mask: Box<dyn GridPredicate>,
    goal: Box<dyn GridPredicate>,
    pub max_speed: i32,
    pub start_r: i32,
    pub start_c: i32,
}

impl GameBoard {
    pub fn new(
        grid: Grid,
        mask: Box<dyn GridPredicate>,
        goal: Box<dyn GridPredicate>,
        max_speed: i32,
        start_r: i32,
        start_c: i32) -> GameBoard
    {
        GameBoard {
            grid,
            mask,
            goal,
            max_speed,
            start_r,
            start_c,
        }
    }
    pub fn is_valid(&self, r: i32, c: i32) -> bool {
        self.mask.test(r, c)
    }

    pub fn is_goal(&self, r: i32, c: i32) -> bool {
        self.goal.test(r, c)
    }

    pub fn generate_circ_track(nr: i32, nc: i32, max_speed: i32) -> Self {
        let grid = Grid { nr, nc };
        let goal = SingletonPredicate::from(1, nc / 2 - 1);
        let mut mask = vec![true; (nr * nc) as usize];
        let (rc, cc) = (nr / 2, nc / 2);
        let r1 = nr as f64 / 4.0;
        let r2 = nr as f64 / 2.0;
        for r in 0..nr {
            for c in 0..nc {
                let d = dist(r, c, rc, cc);
                if d < r1 || r2 < d
                {
                    mask[grid.to_index(r, c)] = false;
                }
            }
        }
        for r in 0..nr / 2 {
            mask[grid.to_index(r, nc / 2)] = false;
        }
        let mask = Mask::from(grid, mask);

        GameBoard::new(grid, Box::new(mask), Box::new(goal), max_speed, 1, nc / 2 + 1)
    }

    pub fn read_csv_file(path: &Path, max_speed: i32) -> Result<Self, Error> {
        let mut data = Vec::<Vec<String>>::new();
        let file = File::open(path)?;
        println!("Reading from file {:?}", path);
        let mut rdr = csv::Reader::from_reader(file);
        let mut size = Option::<usize>::None;
        for result in rdr.records() {
            let mut row = Vec::new();
            let record = result?;
            for element in record.iter() {
                let e = String::from(element);
                row.push(e);
            }
            if size.is_some() {
                if size.unwrap() != row.len() {
                    panic!("Row of different length found");
                }
            } else {
                size = Some(row.len());
            }
            data.push(row);
        }
        Self::read_data(data.into_iter().rev().collect(), max_speed)
    }

    fn read_data(data: Vec<Vec<String>>, max_speed: i32) -> Result<Self, Error> {
        let nr = data.len();
        let nc = data[0].len();
        let grid = Grid { nr: nr as i32, nc: nc as i32 };

        let mut mask = vec![true; nr * nc];
        let mut goal = vec![false; nr * nc];
        let mut start = Option::<(i32, i32)>::None;

        for r in 0..nr as i32 {
            for c in 0..nc as i32 {
                let val = data[r as usize][c as usize].trim().to_lowercase();
                let val = val.as_str();
                match val {
                    "_" => {}
                    "x" => mask[grid.to_index(r, c)] = false,
                    "!" => goal[grid.to_index(r, c)] = true,
                    "s" => if start.is_some() {
                        panic!("Start location must be unique")
                    } else {
                        start = Some((r, c))
                    },
                    _ => panic!("Unrecognized character in data: {}", val.trim()),
                }
            }
        }

        if start.is_some() {
            let start = start.unwrap();
            let mask = Mask::from(grid, mask);
            let goal = Mask::from(grid, goal);
            Ok(GameBoard::new(grid, Box::new(mask), Box::new(goal), max_speed, start.0, start.1))
        } else {
            panic!("Start postion not specified");
        }
    }
}
#[derive(Copy, Clone)]
pub struct Grid {
    pub nr: i32,
    pub nc: i32,
}

impl Grid {
    /// Row major indexing
    pub fn to_index(&self, r: i32, c: i32) -> usize {
        (r * self.nc + c) as usize
    }
    /// Row major indexing
    pub fn to_row_col(&self, i: i32) -> (i32, i32) {
        (i / self.nc, i % self.nc)
    }
}

pub fn dist(r1: i32, c1: i32, r2: i32, c2: i32) -> f64 {
    (((r2 - r1).pow(2) + (c2 - c1).pow(2)) as f64).sqrt()
}
