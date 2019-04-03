#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate sdl2;
extern crate rand;

use std::{thread, time};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use serde_json::Result;
use rocket_contrib::json::{Json, JsonValue};
use rocket::State;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub mod lib;

use lib::cell::{Cell, Grid};

//get cell information via http
#[post("/", data = "<cell>")]
fn create(cell: Json<Cell>, grid_vector: State<Grid>) {

    let color_arr = [cell.red, cell.green, cell.blue];

    let grid = grid_vector.0;

    


    //[cell.row as usize][cell.column as usize] = color_arr;



}

// fn set_color_of_one_cell(grid_vector: Vec<Vec<[u8; 3]>>, cell: Cell) -> Vec<Vec<[u8; 3]>> {
//
//     let color_arr = [cell.red, cell.green, cell.blue];
//     grid_vector[cell.row as usize][cell.column as usize] = color_arr;
//
//     grid_vector
// }


fn main() {

    let (mut renderer, mut events) = lib::init();
    let mut grid_vector = lib::grid_init(lib::NCELLS);



    //http requests
    rocket::ignite()
        .mount("/cell", routes![create])
        .manage(grid_vector)
        .launch();


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
        grid_vector = set_color_of_one_cell(grid_vector, cell);
        grid_vector = lib::next_color_is_random(grid_vector);

        thread::sleep(time::Duration::from_millis(50));
    }
}
