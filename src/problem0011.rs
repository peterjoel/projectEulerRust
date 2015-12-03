/*
Project Euler 0011

In the 20×20 grid below, four numbers along a diagonal line have been marked in red.

08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 ... (data/problem0011.txt)

The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
What is the greatest product of four adjacent numbers in any direction (up, down, left, right,
or diagonally) in the 20×20 grid?

*/
use std::fs::File;
use std::io::Read;
use grid::{Grid,Result};
use grid;

type Item = u8;

pub fn run() -> Result<u32> {
    load_text("data/problem0011_small.txt")
        .and_then( |s| s.parse::<Grid<Item>>() )
        .and_then( largest_product )
}

fn largest_product( grid : Grid<Item> ) -> Result<u32> {
    let run_size = 4;
    // grid.row_iter()
    //     .chain( grid.col_iter() )
    //     .chain( grid.diag_se_iter() )
    //     .chain( grid.diag_sw_iter() )
    grid.rows()
        .chain( grid.cols() )
        .chain( grid.diag_se() )
        .chain( grid.diag_sw() )
        .filter( |row| row.len() >= run_size )
        .flat_map( |row|
                (0..row.len() - run_size + 1)
                    .map( move |i| {
                        row[i .. i + run_size ].iter()
                            .fold(1, |prod,x| prod * (*x as u32))
                    })
        )
        .max()
        .ok_or( grid::err( "The grid was too small." ))
}


fn load_text( location : &str ) -> Result<String> {
    let mut text = String::new();
    File::open( location )
        .and_then( |mut f| f.read_to_string(&mut text))
        .and_then( |_| Ok(text))
}
