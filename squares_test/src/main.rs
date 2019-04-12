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

fn main() {
    let cell = Cell::from_args();
    println!(
        "'{{\"row\":{},\"column\":{},\"red\":{},\"green\":{},\"blue\":{}}}'",
        &cell.row, &cell.column, &cell.red, &cell.blue, &cell.green
    );

    let client = reqwest::Client::new();
    let _res = client.post("http://localhost:8000/cell").json(&cell).send();
}
