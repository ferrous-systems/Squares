extern crate sdl2;
extern crate rand;

use std::{thread, time};

#[macro_use] extern crate serde_derive;


//use serde::{Deserialize, Serialize};
use serde_json::Result;

use rocket_contrib::json::{Json, JsonValue};

use sdl2::pixels::Color;
use sdl2::rect::{Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Renderer;
use sdl2::EventPump;

mod cell;
use cell::{Cell};

mod lib;


fn main() {

    let (mut renderer, mut events) = lib::init();
    let mut grid_vector = lib::grid_init(lib::NCELLS);


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
        //grid_vector = set_color_of_one_cell(grid_vector);
        grid_vector = lib::next_color_is_random(grid_vector);

        thread::sleep(time::Duration::from_millis(50));
    }
}
