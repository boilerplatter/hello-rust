use std::{
    env,
    fmt::{self, Debug, Formatter},
};

enum MyError {
    NoInput,
}

impl Debug for MyError {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            MyError::NoInput => write!(formatter, "Please provide a name for greeting"),
        }
    }
}

fn get_first_argument() -> Result<String, MyError> {
    // this is silly
    env::args().nth(1).ok_or(MyError::NoInput)
}

fn main() -> Result<(), MyError> {
    let caller = get_first_argument()?;

    println!("Hello {}!", &caller);

    Ok(())
}
