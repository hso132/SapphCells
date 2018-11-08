
pub struct Board {
    m_board: Vec<Tile>
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
    fn generate(&self, x: u32, y: u32) -> Tile where Self: Sized {
        Tile::Empty
    }

}
impl NoTerrain {
    pub fn new() -> NoTerrain {NoTerrain}
}

impl Board {
    pub fn new(height: u32, width: u32, genStrat: Box<TerrainGeneratorStrategy>) -> Board {
        let m_board = iproduct!(0..width,0..height).map(|(x,y)| genStrat.generate(x,y)).collect::<Vec<_>>();

        Board {
            m_board
        }
    }
}