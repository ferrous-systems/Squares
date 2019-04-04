#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate sdl2;
extern crate rand;

use std::{thread, time};
use std::sync::{Arc, Mutex};
// use std::sync::atomic::{AtomicUsize, Ordering};
//
// use serde::{Deserialize, Serialize};
// use serde_json::Result;
use rocket_contrib::json::{Json};
use rocket::State;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub mod lib;

use lib::cell::{Cell, Grid};

//get cell information via http
#[post("/", data = "<cell>")]
fn create(cell: Json<Cell>, grid_vector: State<Arc<Mutex<Vec<Vec<[u8; 3]>>>>>) {

    let color_arr = [cell.red, cell.green, cell.blue];

    let mut grid = grid_vector.lock().unwrap();
    grid[cell.row as usize][cell.column as usize] = color_arr;
    println!("{:?}", grid)

}

fn main() {

    let (mut renderer, mut events) = lib::init();
    let grid_vector = lib::grid_init(lib::NCELLS);
    let grid = grid_vector.grid.clone();

    thread::spawn(|| {

        //http requests
        rocket::ignite()
            .mount("/cell", routes![create])
            .manage(grid)
            .launch();

    });



    //video loop
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

        lib::display_frame(&mut renderer, &grid_vector);

        //if no data is comming over http, draw init color


        thread::sleep(time::Duration::from_millis(50));
    }
}
