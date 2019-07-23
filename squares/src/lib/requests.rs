use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};

use crate::lib;
use lib::api::{Cell, Line, ApiGrid};
use lib::data::{SharedGrid, RGB};


//get grid information via http, push entire or partial grids
#[post("/", data = "<grid>")]
pub fn new_grid(grid: Json<ApiGrid>, sharedgrid: State<SharedGrid>) -> JsonValue {
    let mut sharedgrid_data = sharedgrid.sharedgrid.lock().expect("grid lock failed");
    let max_rows = &sharedgrid_data.grid.len();
    let max_columns = &sharedgrid_data.grid[0].len();

    let api_grid = &grid.api_grid;

    for (i, row) in api_grid.iter().enumerate() {
        for (j, _column) in row.iter().enumerate() {

            let cell = Cell {
                row: i as i32 + grid.zero_row ,
                column: j as i32 + grid.zero_column,
                red: api_grid[i][j].red,
                green: api_grid[i][j].green,
                blue: api_grid[i][j].blue,
            };

            let values = lib::err::is_cell_value_in_range(&cell, max_rows, max_columns);
            match values {
                Ok(()) => {
                    let color_arr = RGB {
                        red: cell.red,
                        green: cell.green,
                        blue: cell.blue,
                    };

                    sharedgrid_data.grid[(cell.row) as usize][(cell.column) as usize] = color_arr;
                    json!("success");
                }

                Err(error) => {
                    let response = error.to_string();
                    // println!("{}", &response);
                    json!(response);
                }
            }
        }
    }

    json!("")

}




//get cell information via http, push rgb values in grid
#[post("/", data = "<cell>")]
pub fn add_cell(cell: Json<Cell>, sharedgrid: State<SharedGrid>) -> JsonValue {
    let mut sharedgrid_data = sharedgrid.sharedgrid.lock().expect("grid lock failed");
    let max_rows = &sharedgrid_data.grid.len();
    let max_columns = &sharedgrid_data.grid[0].len();

    let new_cell = Cell {
        row: cell.row,
        column: cell.column,
        red: cell.red,
        green: cell.green,
        blue: cell.blue,
    };

    //checks values
    let values = lib::err::is_cell_value_in_range(&new_cell, max_rows, max_columns);
    match values {
        Ok(()) => {
            let color_arr = RGB {
                red: new_cell.red,
                green: new_cell.green,
                blue: new_cell.blue,
            };
            //del -1
            sharedgrid_data.grid[(new_cell.row) as usize][(new_cell.column) as usize] = color_arr;
            json!("success")
        }

        Err(error) => {
            let response = error.to_string();
            // println!("{}", &response);
            json!(response)
        }
    }
}



//get line information via http, push rgb values in grid
#[post("/", data = "<line>")]
pub fn add_line(line: Json<Line>, sharedgrid: State<SharedGrid>) -> JsonValue {
    let mut sharedgrid_data = sharedgrid.sharedgrid.lock().expect("grid lock failed");
    let max_rows = &sharedgrid_data.grid.len();
    let max_columns = &sharedgrid_data.grid[0].len();

    //checks values
    let values = lib::err::is_line_value_in_range(&line, max_rows, max_columns);
    match values {
        Ok(()) => {

            if line.direction == 1 {
                println!("test1");
                for j in line.row..(line.row + line.length) {
                    let color_arr = RGB {
                        red: line.red,
                        green: line.green,
                        blue: line.blue,
                    };

                    sharedgrid_data.grid[j as usize][line.column as usize] = color_arr;
                }

            } else {
                println!("test2");
                for j in line.column..(line.column + line.length) {
                    let color_arr = RGB {
                        red: line.red,
                        green: line.green,
                        blue: line.blue,
                    };

                    sharedgrid_data.grid[line.row as usize][j as usize] = color_arr;
                }

            }


            json!("success")
        }

        Err(error) => {
            let response = error.to_string();
            // println!("{}", &response);
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
