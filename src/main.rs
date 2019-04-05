#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate sdl2;
extern crate rand;

use std::{thread, time};
use std::sync::{Arc, Mutex};

use rocket_contrib::json::Json;
use rocket::State;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod lib;

use lib::api::Cell;
use lib::data::{RGB, SharedGrid, Grid};

//get cell information via http, push rgb values in grid
#[post("/", data = "<cell>")]
fn create(cell: Json<Cell>, sharedgrid: State<SharedGrid>) {

    let color_arr = RGB { red: cell.red, green: cell.green, blue: cell.blue };

    let mut sharedgrid_data = sharedgrid.sharedgrid.lock().expect("grid lock failed");
    //let mut grid = &sharedgrid_data.grid;

    sharedgrid_data.grid[cell.row as usize][cell.column as usize] = color_arr;
    // println!("{:?}", grid)
}


fn main() {

    let (mut renderer, mut events, video_subsystem) = lib::init();
    let shared_grid = lib::grid_init(lib::NCELLS);
    let sharedgrid_data = shared_grid.sharedgrid.clone();



    thread::spawn(|| {

        //http requests
        //if no data is comming over http, init color is drawn

        rocket::ignite()
            .mount("/cell", routes![create])
            .manage(sharedgrid_data)
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

                Event::KeyDown {
                    keycode: Some(Keycode::Space), ..
                } =>
                        {

                            let mut renderer = lib::set_fullscreen(&video_subsystem);
                            println!("panic!");

                            thread::sleep(time::Duration::from_millis(50));

                            continue 'running

                        },

                _ =>    {}
            }
        }

        lib::display_frame(&mut renderer, &shared_grid);
        thread::sleep(time::Duration::from_millis(50));
    }
}
