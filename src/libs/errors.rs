use std::fmt;
use log::error;
use std::error::Error;
#[derive(Debug)]
pub enum CustomError{
    PARSINGERROR(Box<dyn Error>),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CustomError::*;
        match *self {
            PARSINGERROR(Error) => output_error(*self)
        }
    }
}

pub fn output_error(error: CustomError) -> fmt::Result{
    match error {
        PARSINGERROR => {
            eprintln!();
            error!(
                "There was an error parsing your configuration file: {:#?}",
                error
            );
            panic!("{}", error.to_string())
        }
    }
}