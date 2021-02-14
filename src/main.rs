#![allow(dead_code)]
#![allow(unused_imports)]

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

const ROWS_SIZE: usize = 400;
const COLS_SIZE: usize = 400;

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
	    cells: vec![ Cell::new(); ROWS_SIZE * COLS_SIZE ],
	    mrows: ROWS_SIZE,
	    mcols: COLS_SIZE,
	    state: MapState::Running }
    }
}

pub fn run_sdl()
{
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
                                .position_centered()
                                .build()
                                .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape),
                                 .. } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

pub fn main()
{
    let _map = Map::new();

    run_sdl()
}
