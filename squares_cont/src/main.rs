#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate serde_json;
extern crate structopt;
extern crate rand;

use std::{thread, time};

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
    let mut head_new = Cell::from_args();
    let client = reqwest::Client::new();

    // let mut standardcell = Cell {
    //     row: 0,
    //     column: 0,
    //     red: 35,
    //     green: 15,
    //     blue: 13,
    // };

    let grid = &[(0_i32, 0_i32), (1_i32, 0_i32), (2_i32, 0_i32), (3_i32, 0_i32), (4_i32, 0_i32), (5_i32, 0_i32), (6_i32, 0_i32), (7_i32, 0_i32), (8_i32, 0_i32), (9_i32, 0_i32)];
    let tail_color = &[200_u8, 150_u8, 100_u8, 50_u8, 5_u8];
    let tail_length = &[1_i32, 2_i32, 3_i32, 4_i32];

    let phase_0 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let phase_1 = [200_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let phase_2 = [150_u8, 200_u8, 0, 0, 0, 0, 0, 0, 0, 0];
    let phase_3 = [100_u8, 150_u8, 200_u8, 0, 0, 0, 0, 0, 0, 0];
    let phase_4 = [50_u8, 100_u8, 150_u8, 200_u8, 0, 0, 0, 0, 0, 0];
    let phase_5 = [5_u8, 50_u8, 100_u8, 150_u8, 200_u8, 0, 0, 0, 0, 0];
    let phase_6 = [0, 5_u8, 50_u8, 100_u8, 150_u8, 200_u8, 0, 0, 0, 0];
    let phase_7 = [0, 0, 5_u8, 50_u8, 100_u8, 150_u8, 200_u8, 0, 0, 0];
    let phase_9 = [0, 0, 0, 5_u8, 50_u8, 100_u8, 150_u8, 200_u8, 0, 0];
    let phase_10 = [0, 0, 0, 0, 5_u8, 50_u8, 100_u8, 150_u8, 200_u8, 0];
    let phase_11 = [0, 0, 0, 0, 0, 5_u8, 50_u8, 100_u8, 150_u8, 200_u8];
    let phase_12 = [0, 0, 0, 0, 0, 0, 5_u8, 50_u8, 100_u8, 150_u8];
    let phase_13 = [0, 0, 0, 0, 0, 0, 0, 5_u8, 50_u8, 100_u8];
    let phase_14 = [0, 0, 0, 0, 0, 0, 0, 0, 5_u8, 50_u8];
    let phase_15 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 5_u8];
    let phase_16 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let phase_17 = [0, 0, 0, 0, 0, 0, 0, 0, 0, 200_u8];
    let phase_18 = [0, 0, 0, 0, 0, 0, 0, 0, 200_u8, 150_u8];
    let phase_19 = [0, 0, 0, 0, 0, 0, 0, 200_u8, 150_u8, 100_u8];
    let phase_20 = [0, 0, 0, 0, 0, 0, 200_u8, 150_u8, 100_u8, 50_u8];
    let phase_21 = [0, 0, 0, 0, 0, 200_u8, 150_u8, 100_u8, 50_u8, 5_u8];
    let phase_22 = [0, 0, 0, 0, 200_u8, 150_u8, 100_u8, 50_u8, 5_u8, 0];
    let phase_23 = [0, 0, 0, 200_u8, 150_u8, 100_u8, 50_u8, 5_u8, 0, 0];
    let phase_24 = [0, 0, 200_u8, 150_u8, 100_u8, 50_u8, 5_u8, 0, 0, 0];
    let phase_25 = [0, 200_u8, 150_u8, 100_u8, 50_u8, 5_u8, 0, 0, 0, 0];
    let phase_26 = [200_u8, 150_u8, 100_u8, 50_u8, 5_u8, 0, 0, 0, 0, 0];
    let phase_27 = [150_u8, 100_u8, 50_u8, 5_u8, 0, 0, 0, 0, 0, 0];
    let phase_28 = [100_u8, 50_u8, 5_u8, 0, 0, 0, 0, 0, 0, 0];
    let phase_29 = [50_u8, 5_u8, 0, 0, 0, 0, 0, 0, 0, 0];
    let phase_30 = [5_u8, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let phases = &[phase_0, phase_1, phase_2, phase_3, phase_4, phase_5, phase_6, phase_7, phase_9, phase_10, phase_11, phase_12, phase_13, phase_14,
        phase_15,
        phase_16,
        phase_17,
        phase_18,
        phase_19,
        phase_20,
        phase_21,
        phase_22,
        phase_23,
        phase_24,
        phase_25,
        phase_26,
        phase_27,
        phase_28,
        phase_29,
        phase_30,];

    // let mut head_new = Cell {
    //     row: 1,
    //     column: 0,
    //     red: 200,
    //     green: 0,
    //     blue: 200,
    // };
    // println!(
    //     "'{{\"row\":{},\"column\":{},\"red\":{},\"green\":{},\"blue\":{}}}'",
    //     &cell.row, &cell.column, &cell.red, &cell.blue, &cell.green
    // );
    //let mut cell = movehead(&mut head_new);
    //let _res = client.post("http://localhost:8000/cell").json(&cell).send();

    let tail = [(0_i32, 0_i32), (1_i32, 0_i32), (2_i32, 0_i32)];
    let head = (3_i32, 0_i32);

    loop {
        //let cell = movehead(&mut cell);

        for phase in phases {

            //let _res = client.post("http://localhost:8000/cell").json(&cell).send();
            let _clear = client.get("http://localhost:8000/false").send();
            for (i, position) in grid.iter().enumerate() {

                let snake = makesnake(position, phase[i]);
                let _res = client.post("http://localhost:8000/cell").json(&snake).send();
                //thread::sleep(time::Duration::from_millis(50));

                }

            }
            // if i.0 == cell.row {
            //
            //
            // } else {
            //     standardcell.row = i.0;
            //     //standardcell.column = _i.1;
            //     let _res = client.post("http://localhost:8000/cell").json(&standardcell).send();
            //
            // }

        thread::sleep(time::Duration::from_millis(50));
    }

}


fn movehead (head: &mut Cell) -> &mut Cell {



    if head.row == 10 {
        head.row = 0;
    } else {
        head.row += 1;
    }

    head
}

fn makesnake(position: &(i32, i32), color: u8) -> Cell {

    let tail = Cell {
        row: position.0,
        column: position.1,
        red: color,
        green: 0,
        blue: color,
    };

    tail
}


//
// fn makesnake(mut head: Cell) -> &mut [Cell; 5] {
//
//     let mut snakebody1 = Cell {
//         row: head.row -2,
//         column: head.column,
//         red: 200,
//         green: 0,
//         blue: 200,
//     };
//
//     let mut snakebody2 = Cell {
//         row: head.row -3,
//         column: head.column,
//         red: 180,
//         green: 0,
//         blue: 180,
//     };
//
//     let mut snakebody3 = Cell {
//         row: head.row -4,
//         column: head.column,
//         red: 160,
//         green: 0,
//         blue: 160,
//     };
//
//     let mut snakebody4 = Cell {
//         row: head.row -5,
//         column: head.column,
//         red: 140,
//         green: 0,
//         blue: 140,
//     };
//
//     head.row = -1;
//
//     let snake = &mut [&mut head, &mut snakebody1, &mut snakebody2, &mut snakebody3, &mut snakebody4];
//
//     snake
// }
