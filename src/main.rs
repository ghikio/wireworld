extern crate sdl2;

use sdl2::rect;
use sdl2::mouse;
use sdl2::render;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

/// Indicates the name of the main SDL2 window
const SDL2_WIN_TITLE: &str = "Wireworld";
/// Indicates the width size of the SDL2 window
const SDL2_WIN_WIDTH:  u32 = 516;
/// Indicates the height size of the SDL2 window
const SDL2_WIN_HEIGHT: u32 = 516;

/// Indicates how many cells must be rendered in a row
const CELLS_PER_ROW: usize = 32;
/// Indicates how many cells must be rendered in a column
const CELLS_PER_COL: usize = 32;

/// Indicates the width that each cell must have
const CELL_WIDTH:  usize = SDL2_WIN_WIDTH as usize / CELLS_PER_ROW;
/// Indicates the height that each cell must have
const CELL_HEIGHT: usize = SDL2_WIN_HEIGHT as usize / CELLS_PER_COL;
/// Indicates the border color of the cells
const CELL_BORDER_COLOR: Color = Color::RGB(44, 44, 44);

/// Describes the states in which a `Map` can be
#[derive(Debug, Clone, Copy)]
enum MapState
{
    /// Used to denote that the automaton ticks are paused
    _Stopped,
    /// Used to denote that the automaton ticks are running
    Running,
}

/// Describes the states in which a `Cell` can be
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Clone)]
pub struct Cell {
    /// Indicates the current state of the `Cell`
    state: CellState,
}

impl Cell {
    /// Initializes a new `Cell`
    pub fn new() -> Self {
	Cell { state: CellState::Empty }
    }

    /// Return the color that the cell must have depending on it's
    /// current state
    pub fn get_fill_color(&self) -> Color {
	match self.state {
	    CellState::Empty        => Color::RGB(  0,   0,   0),
	    CellState::ElectronHead => Color::RGB( 58, 126, 191),
	    CellState::ElectronTail => Color::RGB(255,  26,  26),
	    CellState::Conductor    => Color::RGB(255, 255,  85),
	}
    }
}

/// Describes the map and the objects drawn by the graphic engine
#[derive(Debug, Clone)]
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
    /// Initializes a new `Map`
    pub fn new() -> Self {
	Map {
	    cells: vec![ Cell::new(); CELLS_PER_ROW * CELLS_PER_COL ],
	    mrows: CELLS_PER_ROW,
	    mcols: CELLS_PER_COL,
	    state: MapState::Running }
    }

    /// Return a `Cell` as reference
    pub fn get_cell (&self, x: i32, y: i32) -> Option<&Cell> {
	// this check allows us to simplify the syntax on `get_neighbours`
	if x < 0 || y < 0 || x >= CELLS_PER_ROW as i32 || y >= CELLS_PER_COL as i32
	{
	    return None;
	}

	self.cells.get((x as usize * CELLS_PER_ROW) + y as usize)
    }

    /// Return a `Cell` as a mutable reference
    pub fn get_mut_cell (&mut self, x: usize, y: usize) -> Option<&mut Cell> {
	self.cells.get_mut((x * CELLS_PER_ROW) + y)
    }

    pub fn tick(&mut self) {
	let old_gen = self.clone();

	for x in 0 .. CELLS_PER_ROW {
	    for y in 0 .. CELLS_PER_COL {
		let cell       = old_gen.get_cell(x as i32, y as i32).unwrap();
		let neighbours = old_gen.get_neighbours(x, y);

		self.get_mut_cell(x, y).unwrap().state = Self::get_new_state(cell, neighbours);
	    }
	}
    }

    fn get_new_state(cell: &Cell, neighbours: Vec<&Cell>) -> CellState {
	match cell.state {
	    CellState::Empty => CellState::Empty,
	    CellState::ElectronHead => CellState::ElectronTail,
	    CellState::ElectronTail => CellState::Conductor,
	    CellState::Conductor => {
		let total_heads = neighbours.iter().filter(|x| x.state == CellState::ElectronHead).count();

		if total_heads >= 1 && total_heads <= 2 {
		    CellState::ElectronHead
		} else {
		    CellState::Conductor
		}
	    },
	}
    }

    /// Returns a vector of all the `Cell`s contiguous to the position supplied
    /// If some of the adjacent cells are not available, they will not be in the vector
    /// (e.g. the provided cell is on row 1 so there's no upper adjacent cells)
    pub fn get_neighbours (&self, x: usize, y: usize) -> Vec<&Cell> {
	let xpos = x as i32;
	let ypos = y as i32;
	
	vec![
	    // (W) same row left
	    self.get_cell(xpos, ypos - 1),
	    // (E) same row right
	    self.get_cell(xpos, ypos + 1),
	    // (N) upper row center
	    self.get_cell(xpos - 1, ypos),
	    // (NW) upper row left
	    self.get_cell(xpos - 1, ypos - 1),
	    // (NE) upper row right
	    self.get_cell(xpos - 1, ypos + 1),
	    // (S) lower row center
	    self.get_cell(xpos + 1, ypos),
	    // (SW) lower row left
	    self.get_cell(xpos + 1, ypos - 1),
	    // (SE) lower row right
	    self.get_cell(xpos + 1, ypos + 1),
	].into_iter().filter_map(|e| e).collect()
    }
}

/// Renders the cells and other elements into the SDL2 window
pub fn render(canvas: &mut render::WindowCanvas, map: &mut Map)
{
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for x in 0 .. CELLS_PER_ROW {
	let xpos = (x * CELL_HEIGHT) as i32;

	for y in 0 .. CELLS_PER_COL {
	    let ypos = (y * CELL_HEIGHT) as i32;
	    let rect = rect::Rect::new(xpos, ypos, CELL_WIDTH as u32, CELL_HEIGHT as u32);
	    let cell = &map.get_cell(x as i32, y as i32).unwrap();

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

/// Handles the event of left mouse click and performs a modification in the
/// `Cell`'s state at cursor pos.
/// Turns it into a `Conductor` if it was `Empty`, otherwise makes it `Empty`.
pub fn mouse_down_event (map: &mut Map, mouse_btn: mouse::MouseButton, x: i32, y: i32)
{
    let xind = x as usize / CELL_WIDTH;
    let yind = y as usize / CELL_HEIGHT;
    let cell = map.get_mut_cell(xind, yind).unwrap();

    match mouse_btn {
	mouse::MouseButton::Left => {
	    cell.state = match cell.state {
		CellState::Empty     => CellState::Conductor,
		CellState::Conductor => CellState::ElectronHead,
		_                    => CellState::Empty,
	    };
	},
	mouse::MouseButton::Right => cell.state = CellState::Empty,
	_ => ()
    }
}

pub fn run_sdl(map: &mut Map)
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
		Event::MouseButtonDown { mouse_btn, x, y, .. }
		=> mouse_down_event(map, mouse_btn, x, y),
                _ => {}
            }
        }

	// advance generation
	map.tick();

	render(&mut canvas, map);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000 / 10));
    }
}

pub fn main()
{
    let mut map = Map::new();

    run_sdl(&mut map)
}
