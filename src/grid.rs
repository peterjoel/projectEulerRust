use std::result;
use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;

pub type Item = u8;

#[derive(Debug)]
pub struct Grid {
    raw : Vec<Vec<Item>>
}

pub type Result<A> = result::Result<A, Error>;

pub fn err( msg : &str ) -> Error {
    Error::new(ErrorKind::Other, msg.to_owned() )
}

impl Grid {
    pub fn create( data: Vec<Vec<Item>> ) -> Result<Grid> {
        if data.len() == 0 {
            Err( err( "No data!" ))
        }
        else {
            if data.iter().all( |row| row.len() == data[0].len()) {
                Ok(Grid{ raw : data })
            }
            else {
                Err( err("All rows must be same lenth."))
            }
        }
    }

    pub fn width( &self ) -> usize {
        self.rows()[0].len()
    }
    pub fn height( &self ) -> usize {
        self.rows().len()
    }
    pub fn rows( &self ) -> Vec<Vec<Item>> {
        self.raw.to_owned()
    }
    pub fn cols( &self ) -> Vec<Vec<Item>> {
        let mut cols = Vec::new();
        for i in 0..self.height() {
            let col = self.rows().iter()
                        .map( |row| row[i] )
                        .collect::<Vec<Item>>();
            cols.push(col);
        }
        cols
    }

    pub fn diag_se( &self )  -> Vec<Vec<Item>> {
        self.calc_diag( true )
    }

    pub fn diag_sw( &self )  -> Vec<Vec<Item>> {
        self.calc_diag( false )
    }

    fn calc_diag( &self, down_right: bool) -> Vec<Vec<Item>> {

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
    }
}

impl FromStr for Grid {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        parse_grid( s.to_owned() )
    }
}

fn parse_grid( text : String ) -> Result<Grid> {
    text.lines()
            .map( |s| s.to_owned() )
            .map( parse_row )
            .collect::<Result<Vec<_>>>()
            .and_then( Grid::create )
}

fn parse_row( text : String ) -> Result<Vec<Item>> {
    text.split(" ")
        .map(|s| s.to_owned()
            .parse::<Item>()
            .map_err( |e| err( &e.to_string()[..] ))
        )
        .collect::<Result<Vec<Item>>>()
}
