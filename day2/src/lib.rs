use std::error::Error;
use std::fs;

extern crate regex;
use regex::Regex;

pub struct Position {
  pub horizontal: i32,
  pub depth: i32,
}

impl Position {
	pub fn parse(&mut self, contents: &str) -> i32 {
		let result : i32;

		let word_regex = Regex::new(r"\w+").unwrap();
		let num_regex = Regex::new(r"\d+").unwrap();

	    //forward X increases the horizontal position by X units.
    	//down X increases the depth by X units.
    	//up X decreases the depth by X units.

		for line in contents.lines() {
			 
			 let direction;
			 let amount: i32;

			 match word_regex.find(line) {
				Some(x) => direction = &line[x.0 .. x.1],
				None    => unreachable!()
    		 }

			 match num_regex.find(line) {
	        // Get the match slice from string, prints "123"
	        Some(x) => amount = line[x.0 .. x.1].parse().unwrap(),
	        None    => unreachable!()
    		}

    		match direction {
    			"forward" => self.horizontal += amount,
    			"down" => self.depth += amount,
    			"up" => self.depth -= amount,
    			_ => println!("no match"),
    		}
		}

		result = self.horizontal * self.depth;
		result
	}

	pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
		let contents = fs::read_to_string("input")?;

		let mut position = Position{horizontal : 0, depth : 0};
        let answer_day1_part1 = position.parse(&contents);

	    println!("Part one: answer: {}", answer_day1_part1);

	    Ok(())
	}

	    //println!("Result part one: {} \n", increases(&contents));
	    //println!("Result part two: {} \n", solve_part_2(&contents));

	}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example_part_one() {
        let contents = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let mut position = Position{horizontal : 0, depth : 0};
        let answer_day1_part1 = position.parse(contents);
        assert_eq!(15, position.horizontal);
        assert_eq!(10, position.depth);
        assert_eq!(150, answer_day1_part1);
    }
}