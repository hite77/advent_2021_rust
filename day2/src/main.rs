use std::process;

use day2::Position;

fn main() {
    let mut position = Position{horizontal: 0, depth: 0};

    if let Err(e) = position.run() {
        println!("Application error: {}", e);

        process::exit(1);
    }
}
