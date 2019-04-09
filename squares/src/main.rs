#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use] extern crate error_chain;

extern crate rand;
extern crate sdl2;

use sdl2::video::FullscreenType::{self, Desktop, Off};
use std::{thread, time};

use rocket::State;
use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::json;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

//use serde_json::json;

pub mod lib;

use lib::api::Cell;
use lib::data::{SharedGrid, RGB};


//get cell information via http, push rgb values in grid
#[post("/", data = "<cell>")]
fn create(cell: Json<Cell>, sharedgrid: State<SharedGrid>) -> JsonValue {

    //checks values
    let values = lib::err::is_value_in_range(&cell);//.chain_err(|| "test");
    match values {
        Ok(()) => {

            let color_arr = RGB {
                red: cell.red,
                green: cell.green,
                blue: cell.blue,
            };

            let mut sharedgrid_data = sharedgrid.sharedgrid.lock().expect("grid lock failed");
            //let mut grid = &sharedgrid_data.grid;

            sharedgrid_data.grid[cell.row as usize][cell.column as usize] = color_arr;
            // println!("{:?}", grid)

            json!("success")
        }

        Err(error) => {
            let response = error.to_string();
            json!(response)
        }
    }
}

fn main() {
    let (mut canvas, mut events) = lib::new_init();
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
                Event::Quit {..} | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {

                    if canvas.window_mut().fullscreen_state() == FullscreenType::Off {
                        canvas.window_mut().set_fullscreen(Desktop).unwrap();
                        continue 'running

                    } else {
                        canvas.window_mut().set_fullscreen(Off).unwrap();
                        continue 'running
                    };
                }

                _ => continue 'running,
            }
        }

        lib::display_frame(&mut canvas, &shared_grid);
        thread::sleep(time::Duration::from_millis(50));
    }
}
