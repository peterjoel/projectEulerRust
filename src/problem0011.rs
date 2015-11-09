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
use std::result::Result;
use std::io::Read;
use std::io::Error;
use std::io::ErrorKind;
use grid::Grid;

pub fn run() -> Result<u32, Error> {
    load_text("data/problem0011.txt")
        .and_then( |s| s.parse::<Grid>() )
        .and_then( largest_product )
}

fn largest_product( grid : Grid ) -> Result<u32, Error> {
    let run_size = 4;
    let rows = grid.rows();
    rows.iter()
        .chain( grid.cols().iter() )
        .chain( grid.diag_se().iter().filter( |r| r.len() >= run_size ))
        .chain( grid.diag_sw().iter().filter( |r| r.len() >= run_size ))
        .flat_map( |row|
                (0..row.len() - run_size + 1)
                    .map( move |i| {
                        row[i..i+run_size].iter()
                            .fold(1u32, |prod,x| prod * (*x as u32))
                    })
        )
        .max()
        .ok_or(Error::new(ErrorKind::Other, "All rows must be same lenth.".to_owned() ))
}


fn load_text( location : &str ) -> Result<String, Error> {
    let mut s = String::new();
    File::open( location )
        .and_then( |mut f| f.read_to_string(&mut s))
        .and_then( |_| Ok(s))
}
