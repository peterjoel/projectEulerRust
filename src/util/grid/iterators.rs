use std::str::FromStr;
use std::fmt::Debug;
use std::fmt::Display;
use super::model::*;

#[derive(Copy, Clone)]
enum DiagDir {
    DownRight,
    DownLeft,
}


#[derive(Copy, Clone)]
enum GridDir {
    Vertical,
    Horizontal
}


impl <T> Grid <T>
    where T : Copy
{

    pub fn rows( &self ) -> GridRowIter<T> {
        GridRowIter::new( &self, GridDir::Horizontal )
    }

    pub fn cols( &self ) -> GridRowIter<T> {
        GridRowIter::new( &self, GridDir::Vertical )
    }

    pub fn diags_down_right( &self ) -> DiagIter<T> {
        DiagIter::new( &self, DiagDir::DownRight )
    }

    pub fn diags_down_left( &self ) -> DiagIter<T> {
        DiagIter::new( &self, DiagDir::DownLeft )
    }
}


pub struct DiagIter<'a, T:'a>
    where T : Copy
{
    grid : &'a Grid<T>,
    x : i32,
    x_max : i32,
    w : i32,
    h : i32,
    direction : DiagDir,
}

impl <'a,T:'a> DiagIter<'a,T>
    where T : Copy
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


pub struct GridRowIter<'a, T:'a>
    where T : Copy
{
    grid : &'a Grid<T>,
    index : usize,
    direction : GridDir,
}

impl <'a,T:'a> GridRowIter<'a,T>
    where T : Copy
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
    where T : Copy
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            GridDir::Horizontal => {
                if self.index < self.grid.height() {
                    let v = self.grid.raw[self.index].clone();
                    self.index += 1;
                    Some( v )
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
