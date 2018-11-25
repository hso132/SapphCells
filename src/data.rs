extern crate rand;

use self::rand::distributions::*;
use self::rand::thread_rng;
use self::rand::Rng;
use std::collections::HashSet;

pub struct Board {
    m_board: Vec<Tile>,
    m_height: u16,
    m_width: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Empty,
    FoodSource(u32),
    Cell(FoodState),
}

#[derive(Debug, Clone, Copy)]
pub enum FoodState {
    Starving(u32),
    LowOnFood,
    NeutralLow,
    NeutralHigh,
    Surplus(u32),
}

pub trait TerrainGeneratorStrategy {
    fn generate(&self, x: u16, y: u16) -> Tile;
}

pub struct NoTerrain;
pub struct BadRandomTerrain;
pub struct RandomTerrain {
    placed_cells: HashSet<(u16, u16)>,
    food_chance: f32,
}
impl RandomTerrain {
    pub fn new(
        max_cells: u32,
        food_chance: f32,
        max_x: u16,
        max_y: u16,
    ) -> Result<RandomTerrain, String> {
        if max_cells > (max_x as u32 * max_y as u32) {
            Err("Bad Argument; can't generate more cells than space given".to_string())
        } else {
            let mut placed_cells = HashSet::with_capacity(max_cells as usize);
            while placed_cells.len() < max_cells as usize {
                let new_cell = (
                    thread_rng().gen_range(0, max_x),
                    thread_rng().gen_range(0, max_y),
                );
                placed_cells.insert(new_cell);
            }

            Ok(RandomTerrain {
                food_chance,
                placed_cells,
            })
        }
    }
}
impl TerrainGeneratorStrategy for RandomTerrain {
    fn generate(&self, x: u16, y: u16) -> Tile {
        if self.placed_cells.contains(&(x,y)) {
            Tile::Cell(FoodState::LowOnFood)
        } else {
            if self.food_chance > rand::random(){
                Tile::FoodSource(5)
            } else {
                Tile::Empty
            }
        }
    }
}
impl TerrainGeneratorStrategy for NoTerrain {
    fn generate(&self, _x: u16, _y: u16) -> Tile {
        Tile::Empty
    }
}

impl TerrainGeneratorStrategy for BadRandomTerrain {
    fn generate(&self, _x: u16, _y: u16) -> Tile {
        rand::random()
    }
}

impl NoTerrain {
    pub fn new() -> NoTerrain {
        NoTerrain
    }
}
impl BadRandomTerrain {
    pub fn new() -> BadRandomTerrain {
        BadRandomTerrain
    }
}
impl Board {
    pub fn new(m_height: u16, m_width: u16, gen_strat: &TerrainGeneratorStrategy) -> Board {
        let m_board = iproduct!(0..m_width, 0..m_height)
            .map(|(x, y)| gen_strat.generate(x, y))
            .collect::<Vec<_>>();

        Board {
            m_board,
            m_height,
            m_width,
        }
    }
    pub fn board(&self) -> &Vec<Tile> {
        &self.m_board
    }

    pub fn height(&self) -> u16 {
        self.m_height
    }

    pub fn width(&self) -> u16 {
        self.m_width
    }

    pub fn tile_at(&self, x: u16, y: u16) -> Tile {
        self.m_board[(y * self.m_width + x) as usize]
    }
}

impl Distribution<Tile> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Tile {
        use self::Tile::*;
        match rng.gen_range(0, 3) {
            0 => Empty,
            1 => Cell(FoodState::Starving(rng.gen_range(0, 5))),
            2 => FoodSource(1),
            _ => panic!("what th e fuck"),
        }
    }
}
