extern crate rocket;

use crate::lib::api::{Cell, Line};
use rocket_contrib::json::Json;
use std::io::{Error, ErrorKind};

pub mod echain {
    error_chain! {}
}

pub fn is_cell_value_in_range(
    cell: &Cell,
    max_rows: &usize,
    max_columns: &usize,
) -> std::io::Result<()> {

    if cell.column < 0 || cell.column > (*max_columns - 1) as i32 {
        Err(Error::new(ErrorKind::Other, "Column value out of range"))
    } else if cell.row < 0 || cell.row > (*max_rows - 1) as i32 {
        Err(Error::new(ErrorKind::Other, "Row value out of range"))
    } else {
        Ok(())
    }
}

pub fn is_line_value_in_range(
    line: &Json<Line>,
    max_rows: &usize,
    max_columns: &usize,
) -> std::io::Result<()> {


    if line.direction != 1 && line.direction != 0 {
        Err(Error::new(ErrorKind::Other, "Direction must either be horizontal or vertical"))
    // } else if line.column < 0 || line.column > (*max_columns - 1) as i32 {
    //     Err(Error::new(ErrorKind::Other, "Column value of start point is out of range"))
    // } else if line.row < 0 || line.row > (*max_rows - 1) as i32 {
    //     Err(Error::new(ErrorKind::Other, "Row value of start point is out of range"))
    // } else if (line.row + line.length) > (*max_rows - 1) as i32 {
    //     Err(Error::new(ErrorKind::Other, "Part of the line is out of range"))
    // } else if (line.column + line.length) > (*max_rows - 1) as i32 {
    //     Err(Error::new(ErrorKind::Other, "Part of the line is out of range"))
    } else {
        Ok(())
    }
}
