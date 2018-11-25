extern crate sapphcells;

use sapphcells::graphics;
use sapphcells::data::*;
use std::sync::{Mutex,Arc};
fn main() {
    let width = 250;
    let height = 250;
    let strat = RandomTerrain::new(3, 0.01, width, height).unwrap();
    let b = Arc::new(Mutex::new(Board::new(width,height,&strat)));
    graphics::run(b, 1000, 1000);
}
