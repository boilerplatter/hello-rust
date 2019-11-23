use std::env;

fn get_first_argument() -> Option<String> {
    // this is silly
    env::args().nth(1)
}

fn main() {
    let caller = get_first_argument().unwrap_or("world".to_string());

    println!("Hello {}!", &caller);
}
