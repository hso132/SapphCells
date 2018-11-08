extern crate sdl2;

use data::Board;
use std::sync::Mutex;
use std::sync::Arc;
use self::sdl2::pixels::Color;
use self::sdl2::event::Event;
use self::sdl2::video::Window;
use self::sdl2::render::Canvas;

pub fn run(board: Arc<Mutex<Board>>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Sapphie's Cells", 500,500)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0,255,255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => break 'run,
                _ => ()
            }
        }

        update_game(Arc::clone(&board), &mut canvas);

        canvas.present();
    }
}

fn update_game(board: Arc<Mutex<Board>>, canvas: &mut Canvas<Window>) {
    let board = board.lock().unwrap();
    
}