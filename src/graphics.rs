extern crate sfml;
extern crate rand;

use data::*;
use self::sfml::graphics::*;
use self::sfml::window::*;
use self::sfml::system::*;
use std::sync::Mutex;
use std::sync::Arc;
use std::time::Instant;
use std::io::{stdout,Write};


pub fn run(board: Arc<Mutex<Board>>, width: u32, height: u32) {

    let mut window = RenderWindow::new((width,height), "Sapphie's Cells", Style::CLOSE, &Default::default());
    window.set_framerate_limit(60);

    let mut prev = Instant::now();
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => ()
            }
        }

        let mut texture = RenderTexture::new(width, height, false).unwrap();
        update_game(Arc::clone(&board), &mut texture);

        texture.display();
        let mut shape = RectangleShape::new();
        shape.set_size(v2u_to_v2f(texture.size()));
        shape.set_texture(texture.texture(), true);
        window.draw(&shape);
        window.display();
        print!("{} fps\r", 1000.0/prev.elapsed().subsec_millis() as f32);
        stdout().flush().unwrap();
        prev = Instant::now();
    }

}
fn update_game(board: Arc<Mutex<Board>>, canvas: &mut RenderTarget) {
    let board = board.lock().unwrap();
    canvas.clear(&Color::CYAN);
    let Vector2{x: width, y: height} = canvas.size();   
    let (cell_width, cell_height) = (width as f32/board.width() as f32, height as f32/board.height() as f32); 
    for y in 0..board.height() {
        for x in 0..board.width() {
            let tile = board.tile_at(x,y);
            let color = tile_color(tile);
            let mut cell = RectangleShape::with_size(Vector2f{x:cell_width,y:cell_height});
            cell.set_position(Vector2f{x: x as f32*cell_width,y: y as f32*cell_height});
            cell.set_fill_color(&color);
            canvas.draw(&cell);
        }
    }
}
fn tile_color(t: Tile) -> Color {
    match t {
        Tile::Cell(_) => Color::rgb(200,0,0),
        Tile::Empty => Color::rgb(50,50,50),
        Tile::FoodSource(_) => Color::rgb(0,0,200),
    }
}

fn v2u_to_v2f(v1: Vector2u) -> Vector2f {
    Vector2f::from((v1.x as f32,v1.y as f32))
}