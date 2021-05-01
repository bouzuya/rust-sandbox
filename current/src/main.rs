use chrono::prelude::*;
use current::Format;
use std::env;

fn main() {
    let format = env::args().nth(1).expect("no format");
    let format = format.parse::<Format>().expect("unknown format");
    let dt = Local::now().naive_local().date();
    let message = format.format(&dt);
    println!("{}", message);
}
