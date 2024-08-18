use crate::predicates::{GridPredicate, Mask, SingletonPredicate};

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
