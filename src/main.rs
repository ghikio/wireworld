#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sdl2;

use sdl2::rect;
use sdl2::render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const SDL2_WIN_TITLE: &str = "Wireworld";
const SDL2_WIN_WIDTH:  u32 = 1024;
const SDL2_WIN_HEIGHT: u32 = 1024;

/// Indicates how many cells must be rendered in a row
const CELLS_PER_ROW: usize = 16;
/// Indicates how many cells must be rendered in a column
const CELLS_PER_COL: usize = 16;

/// Indicates the width that each cell must have
const CELL_WIDTH:  u32 = SDL2_WIN_WIDTH  / CELLS_PER_ROW as u32;
/// Indicates the height that each cell must have
const CELL_HEIGHT: u32 = SDL2_WIN_HEIGHT / CELLS_PER_COL as u32;

/// Describes the states in which a `Map` can be
enum MapState
{
    /// Used to denote that the automaton ticks are paused
    Stopped,
    /// Used to denote that the automaton ticks are running
    Running,
}

/// Describes the states in which a `Cell` can be
#[derive(Clone)]
enum CellState
{
    /// The cell is dead or doesn't exist
    Empty,
    /// The cell is the head of an electron
    ElectronHead(String),
    /// The cell is the tail of an electron
    ElectronTail(String),
    /// The cell conducts electrons
    Conductor(String),
}

/// Describes a Cell, a thing that can be in different states and
/// interacts with other cells in different ways depending on it's
/// own state.
#[derive(Clone)]
pub struct Cell {
    /// Indicates the current state of the `Cell`
    state: CellState,
}

impl Cell {
    pub fn new() -> Self {
	Cell { state: CellState::Empty }
    }
}

/// Describes the map and the objects drawn by the graphic engine
pub struct Map {
    /// Keeps stored all the cells instanciated in the automaton
    cells: std::vec::Vec<Cell>,
    /// Stores the max number of rows
    mrows: usize,
    /// Stores the max number of columns
    mcols: usize,
    /// Stores the current state of the `Map`
    state: MapState,
}

impl Map {
    pub fn new() -> Self {
	Map {
	    cells: vec![ Cell::new(); CELLS_PER_ROW * CELLS_PER_COL ],
	    mrows: CELLS_PER_ROW,
	    mcols: CELLS_PER_COL,
	    state: MapState::Running }
    }
}

pub fn render(canvas: &mut render::WindowCanvas)
{
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    canvas.set_draw_color(Color::RGB(255, 255, 0));

    let init_width_per_cell = (0..SDL2_WIN_WIDTH)
	.filter(|y| y % CELL_WIDTH == 0)
	.take(CELLS_PER_ROW)
	.collect::<Vec<u32>>();

    let init_height_per_cell = (0..SDL2_WIN_HEIGHT)
	.filter(|x| x % CELL_HEIGHT == 0)
	.take(CELLS_PER_COL)
	.collect::<Vec<u32>>();

    for y in &init_width_per_cell {
	canvas.draw_line(rect::Point::new(0, *y as i32), rect::Point::new(SDL2_WIN_WIDTH as i32, *y as i32))
	    .expect(&format!("couldn't draw line: ({},{}) -> ({},{})", 0, y, SDL2_WIN_WIDTH, y));
    }

    for x in &init_height_per_cell {
	canvas.draw_line(rect::Point::new(*x as i32, 0), rect::Point::new(*x as i32, SDL2_WIN_HEIGHT as i32))
	    .expect(&format!("couldn't draw line: ({},{}) -> ({},{})", x, 0, x, SDL2_WIN_HEIGHT));
    }

    // updates the screen with changes since the last call
    canvas.present();
}

pub fn run_sdl()
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window(SDL2_WIN_TITLE, SDL2_WIN_WIDTH, SDL2_WIN_HEIGHT)
                                .position_centered()
                                .build()
                                .unwrap();

    let mut canvas     = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape),
                                 .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

	render(&mut canvas);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main()
{
    let _map = Map::new();

    run_sdl()
}
