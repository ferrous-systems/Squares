extern crate rocket;

use crate::lib::api::Cell;
use rocket_contrib::json::Json;
use std::io::{Error, ErrorKind};

pub mod echain {
    error_chain! {}
}

pub fn is_value_in_range(
    cell: &Json<Cell>,
    max_rows: &usize,
    max_columns: &usize,
) -> std::io::Result<()> {
    //change max?
    if cell.column < 0 || cell.column > (*max_columns - 1) as i32 {
        Err(Error::new(ErrorKind::Other, "Column value out of range"))
    } else if cell.row < 0 || cell.row > (*max_rows - 1) as i32 {
        Err(Error::new(ErrorKind::Other, "Row value out of range"))
    } else {
        Ok(())
    }
}
