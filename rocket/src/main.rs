#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
use rocket_contrib::json::{Json};

mod db;
mod schema;

mod cell;
use cell::Cell;



//get cell information via http
#[post("/", data = "<cell>")]
fn create(cell: Json<Cell>) -> Cell {

    let cell_data = Cell {
        id: cell.id,
        row: cell.row,
        column: cell.column,
        red: cell.red,
        green: cell.green,
        blue: cell.blue
    };

    cell_data
}

fn main() {

    rocket::ignite()
        .mount("/cell", routes![create])
        .manage(db::connect())
        .launch();

}
