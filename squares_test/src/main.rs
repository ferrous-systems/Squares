#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;
extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(name = "basic")]
struct Cell {
    row: i32,
    column: i32,
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    let cell = Cell::from_args();
    println!(
        "'{{\"row\":{},\"column\":{},\"red\":{},\"green\":{},\"blue\":{}}}'",
        &cell.row, &cell.column, &cell.red, &cell.blue, &cell.green
    );

    let client = reqwest::Client::new();
    let _res = client.post("http://localhost:8000/cell").json(&cell).send();
}
