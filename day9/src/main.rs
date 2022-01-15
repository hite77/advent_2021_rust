use std::process;

fn main() {
    if let Err(e) = day9::run() {
       println!("Application error: {}", e);
       process::exit(1);
    }
}
