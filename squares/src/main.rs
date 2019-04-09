#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

extern crate rand;
extern crate sdl2;

use sdl2::video::FullscreenType::{self, Desktop, Off};
use std::{thread, time};

use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod lib;

use lib::api::Cell;
use lib::data::{SharedGrid, RGB};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct CommandLineArgs {
    rows: i32,
    columns: i32,
}

//get cell information via http, push rgb values in grid
#[post("/", data = "<cell>")]
fn create(cell: Json<Cell>, sharedgrid: State<SharedGrid>) -> JsonValue {

    let mut sharedgrid_data = sharedgrid.sharedgrid.lock().expect("grid lock failed");
    let max_rows = &sharedgrid_data.grid.len();
    let max_columns = &sharedgrid_data.grid[0].len();

    //checks values
    let values = lib::err::is_value_in_range(&cell, max_rows, max_columns);
    match values {
        Ok(()) => {
            let color_arr = RGB {
                red: cell.red,
                green: cell.green,
                blue: cell.blue,
            };



            sharedgrid_data.grid[(cell.row - 1)as usize][(cell.column - 1) as usize] = color_arr;
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


    let args = CommandLineArgs::from_args();

    //init for video loop
    let (canvas_width, canvas_height, cell_width) = lib::determine_canvas_size(args.columns, args.rows);

    let (mut canvas, mut events) = lib::init(canvas_width, canvas_height);
    let shared_grid = lib::grid_init(args.columns, args.rows);
    let sharedgrid_data = SharedGrid {
        sharedgrid: shared_grid.sharedgrid.clone(),
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
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,

                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    //toggle fullscreen
                    if canvas.window_mut().fullscreen_state() == FullscreenType::Off {
                        canvas.window_mut().set_fullscreen(Desktop).unwrap();

                        //change viewport
                        let screen_resolution = lib::get_screen_resolution(&mut canvas);
                        let center_rect =
                            lib::center_rect(screen_resolution.0, screen_resolution.1, canvas_width, canvas_height);

                        canvas.set_viewport(center_rect);
                        continue 'running;
                    } else {
                        canvas.window_mut().set_fullscreen(Off).unwrap();
                        continue 'running;
                    };
                }

                _ => continue 'running,
            }
        }

        lib::display_frame(&mut canvas, &shared_grid, &args.columns, &args.rows, &cell_width);
        thread::sleep(time::Duration::from_millis(50));
    }
}
