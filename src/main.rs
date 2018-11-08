extern crate sapphcells;

use sapphcells::graphics;
use sapphcells::data::*;
use std::sync::{Mutex,Arc};
fn main() {
    let b = Arc::new(Mutex::new(Board::new(5,5,Box::new(NoTerrain::new()))));
    graphics::run(b);
}
