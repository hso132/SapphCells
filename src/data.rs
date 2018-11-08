
pub struct Board {
    m_board: Vec<Tile>,
    m_height: u32,
    m_width: u32
}

pub enum Tile {
    Empty,
    FoodSource(u32),
    Cell(FoodState)
}

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

impl TerrainGeneratorStrategy for NoTerrain {
    fn generate(&self, _x: u32, _y: u32) -> Tile {
        Tile::Empty
    }
}

impl NoTerrain {
    pub fn new() -> NoTerrain {NoTerrain}
}

impl Board {
    pub fn new(m_height: u32, m_width: u32, gen_strat: Box<TerrainGeneratorStrategy>) -> Board {
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
}