use std::process;

fn main() {
    if let Err(e) = day8::run() {
       println!("Application error: {}", e);
       process::exit(1);
    }
}
