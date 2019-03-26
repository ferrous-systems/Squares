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
const CELL_WIDTH: i32 = 20;
const CELL_HEIGHT: i32 = CELL_WIDTH;
const NCELLS: i32 = (MAX_X+1)/CELL_WIDTH;


//creates a grid with ncells*ncells initialized with black cells
fn grid_init(ncells: i32) -> Vec<Vec<Vec<u8>>> {
    let mut rng = rand::thread_rng();

    let mut v:Vec<Vec<Vec<u8>>> = Vec::new();

    let color_vec = vec![0, 0, 0];

    for i in 0..ncells {
        v.push(Vec::new());
        for j in 0..ncells {
            v[i as usize].push(vec![0, 0, 0]);
        }
    }
    v
}

fn random_rgb () -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 255)
}



//converts row column values into xy pixels and draws rectangle
fn display_cell(renderer: &mut Renderer, row: i32, col: i32, v: &Vec<Vec<Vec<u8>>>) {

    let mut x = CELL_WIDTH * col;
    let mut y = CELL_WIDTH * row;

    let cell_color = Color::RGB(v[row as usize][col as usize][0],
                                v[row as usize][col as usize][1],
                                v[row as usize][col as usize][2]);

    renderer.set_draw_color(cell_color);
    renderer.fill_rect(Rect::new(x, y,
                        CELL_WIDTH as u32,
                        CELL_HEIGHT as u32));
}
//displays the whole grid by repeatedly calling display_cell on the alive cells
fn display_frame(r: &mut Renderer, v: &Vec<Vec<Vec<u8>>>) {
    r.set_draw_color(Color::RGB(200, 200, 200));
    r.clear();
    for i in 0..NCELLS {
        for j in 0..NCELLS {
            display_cell(r, i, j, v)
            //if v[i as usize][j as usize] {
            //    display_cell(r, i, j)
            //}
        }
    }
    r.present();
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


fn next_color(v: Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
    let mut v2:Vec<Vec<Vec<u8>>> = Vec::new();

    for i in 0..NCELLS {
        v2.push(Vec::new());
        for j in 0..NCELLS {

            //checks old color
            let rgb = vec![random_rgb(), random_rgb(), random_rgb()];
            v2[i as usize].push(rgb);
            // } else {
            //     v2[i as usize].push(false);
            // }
        };
    }

    v2
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

    let (mut r, mut e) = init(); //better names then r and e?
    let mut v = grid_init(NCELLS);

    'running:loop {
        for event in e.poll_iter() { //e is former event_pump
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape), ..
                } =>
                        { break 'running },
                _ =>    {}
            }
        }

        display_frame(&mut r, &v);
        v = next_color(v);
        println!("{:?}", v);
        thread::sleep(time::Duration::from_millis(50));
    }


}
