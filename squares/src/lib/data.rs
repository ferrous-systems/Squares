use sdl2::render::Canvas;
use sdl2::video::Window;
use std::sync::{Arc, Mutex};

pub struct Grid {
    pub grid: Vec<Vec<RGB>>,
}

pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct SharedGrid {
    pub sharedgrid: Arc<Mutex<Grid>>,
}

pub struct ScreenResolution {
    pub w: i32,
    pub h: i32,
}

pub struct DisplayProperties {
    pub canvas: Canvas<Window>,
    pub rows: i32,
    pub columns: i32,
    pub cell_width: i32,
}
