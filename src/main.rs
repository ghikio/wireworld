#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sdl2;

use sdl2::rect;
use sdl2::mouse;
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
const CELL_WIDTH:  usize = SDL2_WIN_WIDTH as usize / CELLS_PER_ROW;
/// Indicates the height that each cell must have
const CELL_HEIGHT: usize = SDL2_WIN_HEIGHT as usize / CELLS_PER_COL;
/// Indicates the border color of the cells
const CELL_BORDER_COLOR: Color = Color::RGB(255, 255, 255);

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
    ElectronHead,
    /// The cell is the tail of an electron
    ElectronTail,
    /// The cell conducts electrons
    Conductor,
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

    pub fn get_fill_color(&self) -> Color {
	match self.state {
	    CellState::Empty        => Color::RGB(  0,   0,   0),
	    CellState::ElectronHead => Color::RGB(  0,   0, 255),
	    CellState::ElectronTail => Color::RGB(255,   0,   0),
	    CellState::Conductor    => Color::RGB(255, 255,   0),
	}
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

pub fn render(canvas: &mut render::WindowCanvas, map: &Map)
{
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for x in 0 .. CELLS_PER_ROW {
	let xpos = (x * CELL_HEIGHT) as i32;

	for y in 0 .. CELLS_PER_COL {
	    let ypos = (y * CELL_HEIGHT) as i32;
	    let cell = &map.cells[x as usize * y as usize];
	    let rect = rect::Rect::new(xpos, ypos, CELL_WIDTH as u32, CELL_HEIGHT as u32);

	    canvas.set_draw_color(cell.get_fill_color());
	    canvas.fill_rect(rect).unwrap();
	    canvas.set_draw_color(CELL_BORDER_COLOR);
	    canvas.draw_rect(rect)
		.expect(&format!("couldn't draw line: ({},{}) -> ({},{})", 0, y, SDL2_WIN_WIDTH, y));
	}
    }

    // updates the screen with changes since the last call
    canvas.present();
}

pub fn mouse_down_event (x: i32, y: i32)
{
    println!("({:?},{:?})", x, y);
}

pub fn run_sdl(map: Map)
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
		Event::MouseButtonDown { mouse_btn: mouse::MouseButton::Left, x, y, .. }
		=> mouse_down_event(x, y),
                _ => {}
            }
        }
        // The rest of the game loop goes here...

	render(&mut canvas, &map);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main()
{
    let map = Map::new();

    run_sdl(map)
}
