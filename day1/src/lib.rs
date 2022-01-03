use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);
    println!("Result part one: {} \n", increases(&contents));

    Ok(())
}

pub fn increases(contents: &str) -> i32 {
    
    let mut first_line = true;
    let mut count: i32 = 0;
    let mut currentValue: i32 = 0;
    for line in contents.lines() {
        if first_line {
            first_line = false;
            currentValue = line.parse().unwrap();
        } else {
            let read_int: i32 = line.parse().unwrap();
            if read_int > currentValue
            {
                count += 1;
            }
            currentValue = read_int;       
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_increase() {
        let contents = "\
199
200";
        assert_eq!(1, increases(contents));
    }

    #[test]
    fn example_increases() {
        let contents = "\
199
200
208
210
200
207
240
269
260
263";
        assert_eq!(7, increases(contents));
    }
}