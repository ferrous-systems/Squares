use rand::Rng;

use std::sync::{Arc, Mutex};

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Renderer;
use sdl2::EventPump;
use sdl2::VideoSubsystem;

pub mod data;
pub mod api;

use data::Grid;
use data::RGB;

//constants
pub const MAX_X: i32 = 599;
pub const MAX_Y: i32 = MAX_X;
pub const CELL_WIDTH: i32 = 40;
pub const CELL_HEIGHT: i32 = CELL_WIDTH;
pub const NCELLS: i32 = (MAX_X+1)/CELL_WIDTH;


//creates a grid with ncells*ncells initialized with cell in a color
pub fn grid_init(ncells: i32) -> Grid {

    let mut grid_vector:Vec<Vec<RGB>> = Vec::new();

    for row in 0..ncells {
        grid_vector.push(Vec::new());
        for _column in 0..ncells {
            grid_vector[row as usize].push(RGB {red: 35_u8, green: 15_u8, blue: 13_u8});
        }
    }

    let output_grid = Grid {
        grid: Arc::new(Mutex::new(grid_vector))

    };

    output_grid
}


pub fn random_rgb () -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 255)
}


//converts row column values into xy pixels and draws rectangle in the specified color
pub fn display_cell(renderer: &mut Renderer, row: i32, col: i32, grid_vector: &Grid) {

    let grid = grid_vector.grid.lock().expect("53");

    let x = CELL_WIDTH * col;
    let y = CELL_WIDTH * row;

    let cell_color = &grid[row as usize][col as usize];
    let drawing_color = Color::RGB(cell_color.red, cell_color.green, cell_color.blue);

    renderer.set_draw_color(drawing_color);
    renderer.fill_rect(Rect::new(x, y,
                        CELL_WIDTH as u32,
                        CELL_HEIGHT as u32));
}


//displays the whole grid by repeatedly calling display_cell on every cell
pub fn display_frame(renderer: &mut Renderer, grid_vector: &Grid) {

    //let mut grid = grid_vector.grid.lock().unwrap();
    renderer.set_draw_color(Color::RGB(35, 15, 13));
    renderer.clear();

    for row in 0..NCELLS {
        for column in 0..NCELLS {
            display_cell(renderer, row, column, grid_vector)
        }
    }
    renderer.present();
}


pub fn next_color_is_random() -> Vec<Vec<RGB>> {
    let mut new_grid_vector:Vec<Vec<RGB>> = Vec::new();

    for i in 0..NCELLS {
        new_grid_vector.push(Vec::new());
        for _j in 0..NCELLS {

            let rgb = RGB { red: random_rgb(), green: random_rgb(), blue: random_rgb(), };
            new_grid_vector[i as usize].push(rgb);
        };
    }
    new_grid_vector
}


pub fn init<'a>()-> (Renderer<'a>, EventPump, VideoSubsystem) {
    let sdl_context = sdl2::init().expect("sdl init failed");
    let video_subsystem = sdl_context.video().expect("video subsystem failed");

    let window = video_subsystem.window("demo", MAX_X as u32 + 1, MAX_Y as u32 + 1)
        .position_centered()
        .opengl()
        .build()
        .expect("y");

    let mut renderer = window.renderer().build().unwrap();

    let event_pump = sdl_context.event_pump().unwrap();

    renderer.set_draw_color(Color::RGB(35, 15, 13)); //color does not change since being declared here!
    renderer.clear();
    renderer.present();

    (renderer, event_pump, video_subsystem)
}


pub fn set_fullscreen<'a>(video_subsystem: &VideoSubsystem)-> Renderer<'a> {

    let window = video_subsystem.window("demo", MAX_X as u32 + 1, MAX_Y as u32 + 1)
        .position_centered()
        .fullscreen_desktop()
        .opengl()
        .build()
        .expect("156");

    let renderer = window.renderer().build().expect("158");

    renderer


}
