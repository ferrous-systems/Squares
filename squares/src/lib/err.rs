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
    if cell.column < 1 || cell.column > (*max_columns) as i32 {
        Err(Error::new(ErrorKind::Other, "Column value out of range"))
    } else if cell.row < 1 || cell.row > (*max_rows) as i32 {
        Err(Error::new(ErrorKind::Other, "Row value out of range"))
    } else {
        Ok(())
    }
}
