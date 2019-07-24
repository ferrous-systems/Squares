#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;
extern crate structopt;
extern crate rand;

use structopt::StructOpt;
use rand::Rng;

use std::{thread, time};



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
    pub api_grid: [[RGB; 8]; 8],
}

#[derive(StructOpt, Debug, Serialize, Deserialize, Copy, Clone)]
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



fn random_rgb() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, 255)
}

fn make_grid() -> [[RGB; 8]; 8] {

    let color = RGB {
        red: 0,
        green: 0,
        blue: 0,
    };

    let grid: [[RGB; 8]; 8] = [[color; 8]; 8];
    grid
}

fn make_tetris(position: &[[i32; 2]; 4]) -> ApiGrid {

    let mut grid =  make_grid();
    for cell in position {

        let color_arr = RGB {
            red: 100,
            green: 0,
            blue: 100,
        };

        let row =  cell[0] as usize;
        let column = cell[1] as usize;
        grid[row][column] = color_arr;
    }

    let sharedgrid = ApiGrid {
        zero_row: 5,
        zero_column: 5,
        api_grid: grid,
    };

    sharedgrid
}

fn main() {

    let position_1 = [[3, 4], [4, 4], [5, 4], [5, 3]];
    let position_2 = [[4, 5], [4, 4], [4, 3], [3, 3]];
    let position_3 = [[5, 3], [4, 3], [3, 3], [3, 4]];
    let position_4 = [[3, 2], [3, 3], [3, 4], [4, 4]];

    let positions = &[position_1, position_2, position_3, position_4];

    loop {
        for position in positions {
            let sharedgrid = make_tetris(position);
            let client = reqwest::Client::new();

            let grid_sting = serde_json::to_string(&sharedgrid);

            // Print, write to a file, or send to an HTTP server.
            println!("{:?}", grid_sting);

            let _res = client.post("http://localhost:8000/grid").json(&sharedgrid).send();

            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);
        };
    }
}
