use crate::game_space::Grid;

pub trait GridPredicate: Send + Sync {
    fn test(&self, r: i32, c: i32) -> bool;
}


pub struct SingletonPredicate {
    r: i32,
    c: i32,
}

impl GridPredicate for SingletonPredicate {
    fn test(&self, r: i32, c: i32) -> bool {
        self.r == r && self.c == c
    }
}
impl SingletonPredicate {
    pub fn from(r: i32, c: i32) -> SingletonPredicate {
        SingletonPredicate { r, c }
    }
}

pub struct Mask {
    grid: Grid,
    mask: Vec<bool>,
}

impl GridPredicate for Mask {
    fn test(&self, r: i32, c: i32) -> bool {
        self.mask[self.grid.to_index(r, c)]
    }
}

impl Mask {
    pub fn from(grid: Grid, mask: Vec<bool>) -> Mask {
        Mask { grid, mask }
    }
}