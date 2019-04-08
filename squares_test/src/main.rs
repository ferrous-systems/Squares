#[macro_use] extern crate serde_derive;
extern crate structopt;
extern crate reqwest;
extern crate serde_json;


use structopt::StructOpt;

#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(name= "basic")]
struct Cell {
    row: i32,
    column: i32,
    red: u8,
    green: u8,
    blue: u8,
}


//--header "Content-Type: application/json" --request POST --data '{"row":14,"column":0,"red":25,"green":68,"blue":199}' http://localhost:8000/cell
fn main() {
    let cell = Cell::from_args();
    println!("'{{\"row\":{},\"column\":{},\"red\":{},\"green\":{},\"blue\":{}}}'", &cell.row, &cell.column, &cell.red, &cell.blue, &cell.green);
    //let cell_object = format!("{:?}", cell);

    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8000/cell")
        .json(&cell)
        .send();

}
