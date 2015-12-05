use std::result;
use super::error::{err, Error};

pub type Result<A> = result::Result<A, Error>;


#[derive(Debug)]
pub struct Grid<T : Copy> {
    pub raw : Vec<Vec<T>>
}


impl <T : Copy > Grid <T> {

    pub fn create( data: Vec<Vec<T>> ) -> Result<Grid<T>> {
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

}
