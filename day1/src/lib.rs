use std::error::Error;
use std::fs;

pub struct CircularBuffer {
    first: i32,
    second: i32,
    third: i32,
}

impl CircularBuffer {
    pub fn add(&mut self, value: i32) -> i32 {
        self.first = self.second;
        self.second = self.third;
        self.third = value;

        self.first + self.second + self.third
    }
}

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

    println!("Result part one: {} \n", increases(&contents));
    println!("Result part two: {} \n", solve_part_2(&contents));

    Ok(())
}

pub fn increases(contents: &str) -> i32 {
    
    let mut first_line = true;
    let mut count: i32 = 0;
    let mut current_value: i32 = 0;
    for line in contents.lines() {
        if first_line {
            first_line = false;
            current_value = line.parse().unwrap();
        } else {
            let read_int: i32 = line.parse().unwrap();
            if read_int > current_value
            {
                count += 1;
            }
            current_value = read_int;
        }
    }

    return count;
}

pub fn slidingwindow(contents: &str) -> String {
    let mut buffer = CircularBuffer{first: 0, second: 0, third: 0};
    let mut items : i32 = 0;
    let mut current_total: i32;
    let mut result : String = "".to_string();
    
    for line in contents.lines() {
        current_total = buffer.add(line.parse().unwrap());
        items += 1;
        if items >= 3 {
            if result == "" {
                result = format!("{}", current_total.to_string());
            } else {
                result = format!("{}\n{}", result, current_total.to_string());
            }
        }
    }
    result
}

pub fn solve_part_2(contents: &str) -> i32 {
    increases(&slidingwindow(contents))
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

    #[test]
    fn sliding_window() {
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

       let result = "\
607
618
618
617
647
716
769
792";

        assert_eq!(result, slidingwindow(contents));
    }

    #[test]
    fn solve_example_part2() {
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

        assert_eq!(5, solve_part_2(contents));
    }
}