#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate error_chain;

extern crate rand;
extern crate sdl2;

use std::{thread, time};

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use serde_json::json;

pub mod lib;

use lib::api::Cell;
use lib::data::{SharedGrid, RGB};
use lib::err::echain::{Error, ResultExt};


//get cell information via http, push rgb values in grid
#[post("/", data = "<cell>")]
fn create(cell: Json<Cell>, sharedgrid: State<SharedGrid>) -> Result<Json<JsonValue>, Error> {

    //checks values
    let column = lib::err::is_in_column_range(cell.column);
    match column {
        Ok(column) => column,
        Err(error) => {
            println!("he{:?}", error);
        }
    };





    //assert!(cell.row <= 14, "Row value must be between 0 and 14");
    //assert!(cell.row >= 0, "Row value must be between 0 and 14");
    //assert!(cell.column <= 15, "Column value must be between 0 and 14");
    //assert!(cell.column >= 0, "Column value must be between 0 and 14");


    //
    // let color_arr = RGB {
    //     red: cell.red,
    //     green: cell.green,
    //     blue: cell.blue,
    // };
    //
    // let mut sharedgrid_data = sharedgrid.sharedgrid.lock().expect("grid lock failed");
    // //let mut grid = &sharedgrid_data.grid;
    //
    // sharedgrid_data.grid[cell.row as usize][cell.column as usize] = color_arr;
    // // println!("{:?}", grid)
    // res
}

fn main() {
    let (mut renderer, mut events, video_subsystem) = lib::init();
    let shared_grid = lib::grid_init(lib::NCELLS);
    let sharedgrid_data = SharedGrid {
        sharedgrid: shared_grid.sharedgrid.clone()
    };

    thread::spawn(|| {
        //http requests
        //if no data is comming over http, init color is drawn

        rocket::ignite()
            .mount("/cell", routes![create])
            .manage(sharedgrid_data)
            .launch();
    });

    //video loop
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                _ => {}
            }
        }

        lib::display_frame(&mut renderer, &shared_grid);
        thread::sleep(time::Duration::from_millis(50));
    }
}
