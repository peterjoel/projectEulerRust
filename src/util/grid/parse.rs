use std::str::FromStr;
use std::fmt::Display;

use super::model::*;
use super::error::*;

impl <T> FromStr for Grid<T>
    where
        T : Copy + FromStr,
        T::Err : Display
{
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        parse_grid( s.to_owned() )
    }
}

fn parse_grid<T>( text : String ) -> Result<Grid<T>>
    where
        T : Copy + FromStr,
        T::Err : Display
{
    text.lines()
            .map( |s| s.to_owned() )
            .map( parse_row )
            .collect::<Result<Vec<_>>>()
            .and_then( Grid::create )
}

fn parse_row<T>( text : String ) -> Result<Vec<T>>
    where
        T : Copy + FromStr,
        T::Err : Display
{
    text.split(" ")
        .map(|s| s.to_owned()
            .parse::<T>()
            .map_err( |e| err( &e.to_string()[..] ))
        )
        .collect::<Result<Vec<T>>>()
}
