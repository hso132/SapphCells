extern crate sfml;
extern crate rand;

use data::*;
use std::sync::Mutex;
use std::sync::Arc;


pub fn run(board: Arc<Mutex<Board>>) {

    let mut window = RenderWindow::new((500,500), "Sapphie's Cells", Style::CLOSE, &Default::default());

    let mut canvas = window.into_canvas().build().unwrap();



    let mut event_pump = sdl_context.event_pump().unwrap();
    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'run,
                _ => ()
            }
        }

        update_game(Arc::clone(&board), &mut canvas);
    }
}

fn update_game(board: Arc<Mutex<Board>>, canvas: &mut RenderTarget) {
    let board = board.lock().unwrap();
    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    let (width, height) = canvas.output_size().unwrap();   
    let (cell_width, cell_height) = (width/board.width(), height/board.height()); 
    for y in 0..board.height() {
        for x in 0..board.width() {
            let tile = board.tile_at(x,y);
            let color = tile_color(tile);

            canvas.set_draw_color(color);
            canvas.fill_rect(Some(Rect::new((x*cell_width) as i32,(y*cell_height) as i32,cell_width,cell_height))).unwrap();
        }
    }

    canvas.present();
}
fn tile_color(t: Tile) -> Color {
    match t {
        Tile::Cell(_) => Color::RGB(200,0,0),
        Tile::Empty => Color::RGB(50,50,50),
        Tile::FoodSource(_) => Color::RGB(0,0,200),
        _ => random_color(),
    }
}
fn random_color() -> Color {
    Color::RGB(
        rand::random(),
        rand::random(),
        rand::random()
    )
}