use std::result;
use std::io::Error;
use std::io::ErrorKind;
use std::str::FromStr;
use std::fmt::Debug;
use std::fmt::Display;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Grid<Item : Copy> {
    raw : Vec<Vec<Item>>
}

pub type Iter<A> = IntoIter<A>;

pub type Result<A> = result::Result<A, Error>;

pub fn err( msg : &str ) -> Error {
    Error::new(ErrorKind::Other, msg.to_owned() )
}

impl <Item> Grid <Item>
    where Item : Copy + Debug + FromStr + Display
{

    pub fn create( data: Vec<Vec<Item>> ) -> Result<Grid<Item>> {
        if data.len() == 0 {
            Err( err( "No data!" ))
        }
        else {
            if data.iter().all( |row| row.len() == data[0].len()) {
                Ok(Grid{ raw : data })
            }
            else {
                Err( err("All rows must be same length."))
            }
        }
    }

    pub fn width( &self ) -> usize {
        self.raw[0].len()
    }
    pub fn height( &self ) -> usize {
        self.raw.len()
    }

    pub fn rows( &self ) -> Iter<Vec<Item>> {
        self.raw.to_owned().into_iter()
    }

    pub fn cols( &self ) -> Iter<Vec<Item>> {
        let mut cols = Vec::new();
        for i in 0..self.height() {
            let col = self.rows()
                        .map( |row| row[i] )
                        .collect::<Vec<Item>>();
            cols.push(col);
        }
        cols.into_iter()
    }

    pub fn diag_se( &self ) -> Iter<Vec<Item>> {
        self.calc_diag( true )
    }

    pub fn diag_sw( &self ) -> Iter<Vec<Item>> {
        self.calc_diag( false )
    }

    fn calc_diag( &self, down_right: bool) -> Iter<Vec<Item>> {

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
        diag.into_iter()
    }
}

impl <Item> FromStr for Grid<Item>
    where
        Item : Copy + Debug + Display + FromStr,
        Item::Err : Display
{
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        parse_grid( s.to_owned() )
    }
}

fn parse_grid<Item>( text : String ) -> Result<Grid<Item>>
    where
        Item : Copy + Debug + Display + FromStr,
        Item::Err : Display
{
    text.lines()
            .map( |s| s.to_owned() )
            .map( parse_row )
            .collect::<Result<Vec<_>>>()
            .and_then( Grid::create )
}

fn parse_row<Item>( text : String ) -> Result<Vec<Item>>
    where
        Item : Copy + Debug + Display + FromStr,
        Item::Err : Display
{
    text.split(" ")
        .map(|s| s.to_owned()
            .parse::<Item>()
            .map_err( |e| err( &e.to_string()[..] ))
        )
        .collect::<Result<Vec<Item>>>()
}
