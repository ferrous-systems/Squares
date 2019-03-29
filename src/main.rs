extern crate sdl2;
extern crate rand;

use std::{thread, time};

use rand::Rng;

use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;

//constants
const MAX_X: i32 = 599;
const MAX_Y: i32 = MAX_X;
const CELL_WIDTH: i32 = 40;
const CELL_HEIGHT: i32 = CELL_WIDTH;
const NCELLS: i32 = (MAX_X+1)/CELL_WIDTH;

mod cell;
use cell::{Cell};


//creates a grid with ncells*ncells initialized with cell in caput mortuum
fn grid_init(ncells: i32) -> Vec<Vec<[u8; 3]>> {
    //let mut rng = rand::thread_rng();

    let mut grid_vector:Vec<Vec<[u8; 3]>> = Vec::new();

    let color_arr = [35_u8, 15_u8, 13_8];

    for row in 0..ncells {
        grid_vector.push(Vec::new());
        for column in 0..ncells {
            grid_vector[row as usize].push(color_arr); // why doesnt color_vec work in there?
        }
    }

    grid_vector
}

fn random_rgb () -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 255)
}

//converts row column values into xy pixels and draws rectangle
fn display_cell(renderer: &mut Renderer, row: i32, col: i32, grid_vector: &Vec<Vec<[u8; 3]>>) {

    let mut x = CELL_WIDTH * col;
    let mut y = CELL_WIDTH * row;
    println!("{}", row);

    let cell_color = Color::RGB(grid_vector[row as usize][col as usize][0],
                                grid_vector[row as usize][col as usize][1],
                                grid_vector[row as usize][col as usize][2]);

    renderer.set_draw_color(cell_color);
    renderer.fill_rect(Rect::new(x, y,
                        CELL_WIDTH as u32,
                        CELL_HEIGHT as u32));
}


//displays the whole grid by repeatedly calling display_cell on the alive cells
fn display_frame(renderer: &mut Renderer, grid_vector: &Vec<Vec<[u8; 3]>>) {
    renderer.set_draw_color(Color::RGB(200, 200, 200));
    renderer.clear();
    for row in 0..NCELLS {
        for column in 0..NCELLS {
            display_cell(renderer, row, column, grid_vector)
            //if v[i as usize][j as usize] {
            //    display_cell(r, i, j)
            //}
        }
    }

    renderer.present();
}


// //checks old color
// fn new_color(r: i32, c: i32, v: &Vec<Vec<Vec<u8>>>) -> u8 {
//
//     let n = count_surrounding(r, c, v);
//
//     for i in v[r as usize][c as usize] {
//         println!("{}", i);
//     }
//
//
//
//
//
// }
fn set_color_of_one_cell(grid_vector: Vec<Vec<[u8; 3]>>, cell_info: Cell) -> Vec<Vec<[u8; 3]>> {

    let color_arr = [cell_info.red, cell_info.green, cell_info.blue];


    grid_vector[cell_info.row as usize][cell_info.column as usize] = color_arr; // why doesnt color_vec work in there?

    grid_vector

}


fn next_color_is_random(grid_vector: Vec<Vec<[u8; 3]>>) -> Vec<Vec<[u8; 3]>> {
    let mut new_grid_vector:Vec<Vec<[u8; 3]>> = Vec::new();
    //Right now, this only creates new random colors
    for i in 0..NCELLS {
        new_grid_vector.push(Vec::new());
        for j in 0..NCELLS {

            //checks old color
            let rgb = [random_rgb(), random_rgb(), random_rgb()];
            new_grid_vector[i as usize].push(rgb);
            // } else {
            //     v2[i as usize].push(false);
            // }
        };
    }

    new_grid_vector
}

fn init<'a>()-> (Renderer<'a>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("demo", MAX_X as u32 + 1, MAX_Y as u32 + 1)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut renderer = window.renderer().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    renderer.set_draw_color(Color::RGB(0, 0, 0)); //color does not change since being declared here!
    renderer.clear();
    renderer.present();

    (renderer, event_pump)

}

fn main() {

    let (mut renderer, mut events) = init();
    let mut grid_vector = grid_init(NCELLS);

    'running:loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } =>
                        { break 'running },
                _ =>    {}
            }
        }

        display_frame(&mut renderer, &grid_vector);
        grid_vector = next_color_is_random(grid_vector);

        thread::sleep(time::Duration::from_millis(50));
    }
}
