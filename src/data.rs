extern crate rand;

use self::rand::distributions::*;
use self::rand::Rng;

pub struct Board {
    m_board: Vec<Tile>,
    m_height: u32,
    m_width: u32
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Empty,
    FoodSource(u32),
    Cell(FoodState)
}

#[derive(Debug, Clone, Copy)]
pub enum FoodState {
    Starving,
    LowOnFood,
    NeutralLow,
    NeutralHigh,
    Surplus(u32)
}

pub trait TerrainGeneratorStrategy {
    fn generate(&self, x: u32, y: u32) -> Tile;
}

pub struct NoTerrain;
pub struct BadRandomTerrain;

impl TerrainGeneratorStrategy for NoTerrain {
    fn generate(&self, _x: u32, _y: u32) -> Tile {
        Tile::Empty
    }
}

impl TerrainGeneratorStrategy for BadRandomTerrain {
    fn generate(&self, _x: u32, _y: u32) -> Tile {
        rand::random()
    }
}

impl NoTerrain {
    pub fn new() -> NoTerrain {NoTerrain}
}
impl BadRandomTerrain {
    pub fn new() ->BadRandomTerrain {BadRandomTerrain}
}
impl Board {
    pub fn new(m_height: u32, m_width: u32, gen_strat: &TerrainGeneratorStrategy) -> Board {
        let m_board = iproduct!(0..m_width,0..m_height).map(|(x,y)| gen_strat.generate(x,y)).collect::<Vec<_>>();

        Board {
            m_board, m_height, m_width
        }
    }
    pub fn board(&self) -> &Vec<Tile> {
        &self.m_board
    }

    pub fn height(&self) -> u32 {
        self.m_height
    }

    pub fn width(&self) -> u32 {
        self.m_width
    }

    pub fn tile_at(&self, x: u32, y: u32) -> Tile {
        self.m_board[(y*self.m_width+x) as usize]
    }
}

impl Distribution<Tile> for Standard {
    fn sample <R: Rng + ?Sized>(&self, rng: &mut R) -> Tile {
        use self::Tile::*;
        match rng.gen_range(0, 3) {
            0 => Empty,
            1 => Cell(FoodState::Starving),
            2 => FoodSource(1),
            _ => panic!("what th e fuck")
        }
    }
}