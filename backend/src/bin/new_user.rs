extern crate diesel;

use backlib::*;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();
    let args: Vec<String> = std::env::args().collect();
    create_user(
        &connection,
        args.get(1).unwrap(),
        args.get(2).unwrap(),
        args.get(3).unwrap(),
    );
}
