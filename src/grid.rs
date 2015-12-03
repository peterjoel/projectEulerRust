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
    //
    //
    // pub fn corners( &self ) -> CornersIter<Item> {
    //     CornersIter::new( &self )
    // }


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
        self.calc_diag( DiagDir::DownRight )
    }

    pub fn diag_sw( &self ) -> Iter<Vec<Item>> {
        self.calc_diag( DiagDir::DownLeft )
    }

    fn calc_diag( &self, direction:DiagDir) -> Iter<Vec<Item>> {

        let w = self.width() as i32;
        let h = self.height() as i32;
        let mut diag = Vec::new();
        let x_starts = match direction {
            DiagDir::DownRight => 1-h..w,
            DiagDir::DownLeft => 0..w+h-1,
        };

        for x in x_starts {
            let mut diag_line = Vec::new();
            for y in 0..h {
                let src_x = match direction {
                    DiagDir::DownRight => x + y,
                    DiagDir::DownLeft => x - y,
                };
                let src_y = y;
                if src_x >= 0 && src_x < w {
                    diag_line.push(self.raw[src_y as usize][src_x as usize]);
                }
            }
            diag.push( diag_line );
        }
        diag.into_iter()
    }

    pub fn row_iter( &self ) -> GridRowIter<Item> {
        GridRowIter::new( &self, GridDir::Horizontal )
    }


    pub fn col_iter( &self ) -> GridRowIter<Item> {
        GridRowIter::new( &self, GridDir::Vertical )
    }


    pub fn diag_se_iter( &self ) -> DiagIter<Item> {
        DiagIter::new( &self, DiagDir::DownRight )
    }

    pub fn diag_sw_iter( &self ) -> DiagIter<Item> {
        DiagIter::new( &self, DiagDir::DownLeft )
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

#[derive(Copy, Clone)]
enum DiagDir {
    DownRight,
    DownLeft,
}


pub struct DiagIter<'a, T:'a>
    where T : Copy + Debug + Display + FromStr
{
    grid : &'a Grid<T>,
    x : i32,
    x_max : i32,
    w : i32,
    h : i32,
    direction : DiagDir,
}

impl <'a,T:'a> DiagIter<'a,T>
    where T : Copy + Debug + Display + FromStr
{
    fn new( grid : &'a Grid<T>, direction : DiagDir ) -> DiagIter<'a,T> {

        let w = grid.width() as i32;
        let h = grid.height() as i32;
        DiagIter {
            grid : grid,
            x : match direction {
                DiagDir::DownRight => 1 - h,
                DiagDir::DownLeft => 0,
            },
            x_max : match direction {
                DiagDir::DownRight => w,
                DiagDir::DownLeft => h + w - 1,
            },
            w : w,
            h : h,
            direction : direction,
        }
    }
}

impl <'a,T> Iterator for DiagIter<'a,T>
    where T : Copy + Debug + Display + FromStr
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.x_max {
            None
        }
        else {
            let mut v = Vec::new();

            for y in 0..self.h {
                let src_x = match self.direction {
                    DiagDir::DownRight => self.x + y,
                    DiagDir::DownLeft => self.x - y,
                };
                if src_x >= 0 && src_x < self.w {
                    v.push(self.grid.raw[y as usize][src_x as usize]);
                }
            }

            self.x += 1;

            Some( v )
        }
    }
}

#[derive(Copy, Clone)]
enum GridDir {
    Vertical,
    Horizontal
}

pub struct GridRowIter<'a, T:'a>
    where T : Copy + Debug + Display + FromStr
{
    grid : &'a Grid<T>,
    index : usize,
    direction : GridDir,
}

impl <'a,T:'a> GridRowIter<'a,T>
    where T : Copy + Debug + Display + FromStr
{
    fn new( grid : &'a Grid<T>, direction: GridDir ) -> GridRowIter<'a,T> {
        GridRowIter {
            grid : grid,
            index : 0,
            direction : direction,
        }
    }
}

impl <'a,T> Iterator for GridRowIter<'a,T>
    where T : Copy + Debug + Display + FromStr
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            GridDir::Horizontal => {
                if self.index < self.grid.height() {
                    let r = Some( self.grid.raw[self.index].clone() );
                    self.index += 1;
                    r
                }
                else {
                    None
                }
            },
            GridDir::Vertical => {
                if self.index < self.grid.height() {
                    let mut v = Vec::new();
                    for row in self.grid.raw.iter() {
                        v.push( row[self.index] );
                    }
                    self.index += 1;
                    Some( v )
                }
                else {
                    None
                }
            }
        }

    }
}




//
//
// pub struct CornersIter<'a, T:'a>
//     where T : Copy + Debug + Display + FromStr
// {
//     grid : &'a Grid<T>,
//     index : usize,
// }
//
// impl <'a,T:'a> CornersIter<'a,T>
//     where T : Copy + Debug + Display + FromStr
// {
//     fn new( grid : &'a Grid<T> ) -> CornersIter<'a,T> {
//         CornersIter {
//             grid : grid,
//             index : 0
//         }
//     }
// }
//
// impl <'a,T> Iterator for CornersIter<'a,T>
//     where T : Copy + Debug + Display + FromStr
// {
//     type Item = T;
//
//     fn next(&mut self) -> Option<T> {
//         let r = match self.index {
//             0 => Some(self.grid.raw[0][0]),
//             1 => Some(self.grid.raw[self.grid.height()-1][0]),
//             2 => Some(self.grid.raw[0][self.grid.width()-1]),
//             3 => Some(self.grid.raw[self.grid.height()-1][self.grid.width()-1]),
//             _ => None,
//         };
//         self.index += 1;
//         r
//     }
// }
