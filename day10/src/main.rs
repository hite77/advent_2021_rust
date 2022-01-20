use std::process;

fn main() {
    if let Err(e) = day10::run() {
       println!("Application error: {}", e);
       process::exit(1);
    }
}
