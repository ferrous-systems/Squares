use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use crate::lib;
use lib::api::Cell;
use lib::data::{SharedGrid, RGB};

//get cell information via http, push rgb values in grid
#[post("/", data = "<cell>")]
pub fn change_grid(cell: Json<Cell>, sharedgrid: State<SharedGrid>) -> JsonValue {
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
            //del -1
            sharedgrid_data.grid[(cell.row) as usize][(cell.column) as usize] = color_arr;
            json!("success")
        }

        Err(error) => {
            let response = error.to_string();
            json!(response)
        }
    }
}

#[get("/intervention/<intervention>")]
pub fn intervention(
    intervention: bool,
    sharedgrid: State<SharedGrid>,
    program_paused: State<Arc<AtomicBool>>,
) -> JsonValue {
    lib::clear_grid(&sharedgrid);

    if intervention {
        lib::make_checker_board(&sharedgrid);

        thread::sleep(time::Duration::from_millis(100));
        program_paused.store(intervention, Ordering::Relaxed);
        json!("paused")
    } else {
        thread::sleep(time::Duration::from_millis(50));
        program_paused.store(intervention, Ordering::Relaxed);
        json!("unpaused")
    }
}
