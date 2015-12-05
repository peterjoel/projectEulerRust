pub use std::io::Error as Error;
use std::io::ErrorKind;

pub fn err( msg : &str ) -> Error {
    Error::new(ErrorKind::Other, msg.to_owned() )
}
