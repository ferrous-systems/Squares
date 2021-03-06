#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;
extern crate structopt;
extern crate rand;

use structopt::StructOpt;
use rand::Rng;

#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(name = "basic")]
struct Cell {
    row: i32,
    column: i32,
    red: u8,
    green: u8,
    blue: u8,
}
#[derive(StructOpt, Debug, Serialize, Deserialize)]
#[structopt(name = "basic")]
pub struct Line {
    pub row: i32,
    pub column: i32,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub direction: i32,
    pub length: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiGrid {
    pub zero_row: i32,
    pub zero_column: i32,
    pub api_grid: Vec<Vec<RGB>>,
}

#[derive(StructOpt, Debug, Serialize, Deserialize)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

// pub fn next_color_is_random() -> Vec<Vec<RGB>> {
//     let mut new_grid_vector: Vec<Vec<RGB>> = Vec::new();
//
//     for i in 0..NCELLS {
//         new_grid_vector.push(Vec::new());
//         for _j in 0..NCELLS {
//             let rgb = RGB {
//                 red: random_rgb(),
//                 green: random_rgb(),
//                 blue: random_rgb(),
//             };
//             new_grid_vector[i as usize].push(rgb);
//         }
//     }
//     new_grid_vector
//}



pub fn random_rgb() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 255)
}

pub fn make_grid() -> Vec<Vec<RGB>> {
    let mut grid_vec = Vec::new();
    for _i in 0..2 {
        let mut rows_vec = Vec::new();
        for _j in 0..2 {
            let color = RGB {
                red: 23,
                green: random_rgb(),
                blue: random_rgb(),
            };
            rows_vec.push(color)
        }

        grid_vec.push(rows_vec)
    }

    grid_vec
}

fn main() {

    let line = Line::from_args();


    // Print, write to a file, or send to an HTTP server.


    println!(
        "'{{\"row\":{},\"column\":{},\"red\":{},\"green\":{},\"blue\":{},\"direction\":{},\"length\":{}}}'",
        &line.row, &line.column, &line.red, &line.blue, &line.green, &line.direction, &line.length
    );

    // let cell = Cell::from_args();
    // println!(
    //     "'{{\"row\":{},\"column\":{},\"red\":{},\"green\":{},\"blue\":{}}}'",
    //     &cell.row, &cell.column, &cell.red, &cell.blue, &cell.green
    // );
    //
    let client = reqwest::Client::new();
    let _res = client.post("http://localhost:8000/line").json(&line).send();
}
