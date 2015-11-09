use std::result::Result;
use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;
// use std::cmp::{min, max};

pub type Item = u8;
pub type Row = Vec<Item>;

#[derive(Debug)]
pub struct Grid {
    raw : Vec<Row>
}

impl Grid {
    pub fn create( data: Vec<Row> ) -> Result<Grid, Error> {
        if data.len() == 0 {
            Err(Error::new(ErrorKind::Other, "No data!".to_owned()))
        }
        else {
            if data.iter().all( |row| row.len() == data[0].len()) {
                Ok(Grid{ raw : data })
            }
            else {
                Err( Error::new(ErrorKind::Other, "All rows must be same lenth.".to_owned() ))
            }
        }
    }

    pub fn width( &self ) -> usize {
        self.rows()[0].len()
    }
    pub fn height( &self ) -> usize {
        self.rows().len()
    }
    pub fn rows( &self ) -> Vec<Row> {
        self.raw.to_owned()
    }
    pub fn cols( &self ) -> Vec<Row> {
        let mut cols = Vec::new();
        for i in 0..self.height() {
            let col = self.rows().iter()
                        .map( |row| row[i] )
                        .collect::<Row>();
            cols.push(col);
        }
        cols
    }

    pub fn diag_se( &self )  -> Vec<Row> {
        self.calc_diag( true )
    }

    pub fn diag_sw( &self )  -> Vec<Row> {
        self.calc_diag( false )
    }

    fn calc_diag( &self, down_right: bool) -> Vec<Row> {

        let w = self.width() as i32;
        let h = self.height() as i32;

        let mut diag = Vec::new();

        let x_starts = if down_right { 1-h..w } else { 0..w+h-1 };

        for x in x_starts {
            let mut diag_line = Vec::new();
            for y in 0..h {
                let src_x = if down_right { x+y } else { x-y };
                let src_y = y;
                if src_x >= 0 && src_x < w {
                    diag_line.push(self.raw[src_y as usize][src_x as usize]);
                }
            }
            diag.push( diag_line );
        }


        diag


    //    1 2 3 4
    //    1 2 3 4
    //    1 2 3 4
    // diagonal \
    //         . . 1 2 3 4
    //         . 1 2 3 4 .
    //         1 2 3 4 . .
    // diagonal /
    //         1 2 3 4 . .
    //         . 1 2 3 4 .
    //         . . 1 2 3 4

    }
}

impl FromStr for Grid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_grid( s.to_owned() )
    }
}

fn parse_grid( text : String ) -> Result<Grid, Error> {
    text.lines()
            .map( |s| s.to_owned() )
            .map( parse_row )
            .collect::<Result<Vec<_>, _>>()
            .and_then( Grid::create )
}

fn parse_row( text : String ) -> Result<Row, Error> {
    text.split(" ")
        .map(|s| s.to_owned()
            .parse::<Item>()
            .map_err( |e| Error::new(ErrorKind::Other, e.to_string() ))
        )
        .collect::<Result<Row,_>>()
}
