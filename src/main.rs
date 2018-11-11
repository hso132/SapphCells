extern crate sapphcells;

use sapphcells::graphics;
use sapphcells::data::*;
use std::sync::{Mutex,Arc};
fn main() {
    let b = Arc::new(Mutex::new(Board::new(490,500,&BadRandomTerrain::new())));
    graphics::run(b);
}
