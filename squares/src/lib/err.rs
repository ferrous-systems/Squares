extern crate rocket;

use crate::lib::api::{Cell, Line, ApiGrid};
use rocket_contrib::json::Json;
use std::io::{Error, ErrorKind};

pub mod echain {
    error_chain! {}
}

pub fn does_grid_fit(grid: &Json<ApiGrid>, max_rows: &usize, max_columns: &usize) -> std::io::Result<()> {
    let api_grid = &grid.api_grid;
    let grid_rows = &api_grid.len();
    let grid_columns = &api_grid[0].len();


    let row = *&grid.zero_row as usize;
    let column = *&grid.zero_column as usize;

    println!("{}", (row + grid_rows));
    if grid_rows > max_rows  {
        Err(Error::new(ErrorKind::Other, "The grid's hight is too big."))
    } else if grid_columns > max_columns {
        Err(Error::new(ErrorKind::Other, "The grid's width is too wide."))
    } else if (row + grid_rows) > *max_rows {
        Err(Error::new(ErrorKind::Other, "Some of the grid's rows are out of range."))
    } else if (column + grid_columns) > *max_columns {
        Err(Error::new(ErrorKind::Other, "Some of the grid's columns are out of range."))
    } else {
        Ok(())
    }
}

pub fn is_value_in_range(
    cell: &Cell,
    max_rows: &usize,
    max_columns: &usize,
) -> std::io::Result<()> {
    //change max?
    if cell.column < 0 || cell.column > (*max_columns) as i32 {
        Err(Error::new(ErrorKind::Other, "Column value out of range"))
    } else if cell.row < 0 || cell.row > (*max_rows) as i32 {
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
    println!("test3");

    if line.direction != 1 && line.direction != 0 {
        Err(Error::new(ErrorKind::Other, "Direction must either be horizontal or vertical"))
    } else if line.column < 0 || line.column > (*max_columns - 1) as i32 {
        Err(Error::new(ErrorKind::Other, "Column value of start point is out of range"))
    } else if line.row < 0 || line.row > (*max_rows - 1) as i32 {
        Err(Error::new(ErrorKind::Other, "Row value of start point is out of range"))
    } else if (line.row + line.length) > (*max_rows - 1) as i32 {
        Err(Error::new(ErrorKind::Other, "Part of the line is out of range"))
    } else if (line.column + line.length) > (*max_rows - 1) as i32 {
        Err(Error::new(ErrorKind::Other, "Part of the line is out of range"))
    } else {
        Ok(())
    }
}
