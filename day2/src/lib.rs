use std::error::Error;
use std::fs;

extern crate regex;
use regex::Regex;

pub struct Position {
  pub horizontal: i32,
  pub depth: i32,
}

pub struct Position2 {
  pub horizontal: i32,
  pub depth: i32,
  aim: i32	
}

impl Position2 {
	pub fn parse(&mut self, contents: &str) -> i32 {
				let word_regex = Regex::new(r"\w+").unwrap();
		let num_regex = Regex::new(r"\d+").unwrap();

	    //forward X increases the horizontal position by X units.
    	//down X increases the depth by X units.
    	//up X decreases the depth by X units.

		for line in contents.lines() {
			 
			 let amount: i32;
			 
			 match num_regex.find(line) {
	        // Get the match slice from string, prints "123"
	        Some(x) => amount = line[x.0 .. x.1].parse().unwrap(),
	        None    => unreachable!()
    		}

    		// down increases aim 

    		match word_regex.find(line) {
				None    => unreachable!(),
				Some(x) => match &line[x.0 .. x.1] {
    			"forward" => {
    				self.horizontal += amount;
    				self.depth += self.aim * amount;
    			},
    			"down" => self.aim += amount,
    			"up" => self.aim -= amount,
    			_ => println!("no match"),					
				},
			}
		}
		self.horizontal * self.depth
	}

	}

impl Position {
	pub fn parse(&mut self, contents: &str) -> i32 {
		let word_regex = Regex::new(r"\w+").unwrap();
		let num_regex = Regex::new(r"\d+").unwrap();

	    //forward X increases the horizontal position by X units.
    	//down X increases the depth by X units.
    	//up X decreases the depth by X units.

		for line in contents.lines() {
			 
			 let amount: i32;
			 
			 match num_regex.find(line) {
	        // Get the match slice from string, prints "123"
	        Some(x) => amount = line[x.0 .. x.1].parse().unwrap(),
	        None    => unreachable!()
    		}

    		match word_regex.find(line) {
				None    => unreachable!(),
				Some(x) => match &line[x.0 .. x.1] {
    			"forward" => self.horizontal += amount,
    			"down" => self.depth += amount,
    			"up" => self.depth -= amount,
    			_ => println!("no match"),					
				},
			}
		}
		self.horizontal * self.depth
	}

	pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
		let contents = fs::read_to_string("input")?;

		let mut position = Position{horizontal : 0, depth : 0};
        let answer_day1_part1 = position.parse(&contents);
        let answer_day2_part2 = Position2{horizontal: 0, depth: 0, aim: 0}.parse(&contents);

	    println!("Part one: answer: {}", answer_day1_part1);
	    println!("Part two: answer: {}", answer_day2_part2);

	    Ok(())
	}

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
        let answer_day2_part1 = position.parse(contents);
        assert_eq!(15, position.horizontal);
        assert_eq!(10, position.depth);
        assert_eq!(150, answer_day2_part1);
    }

    // In addition to horizontal position and depth, you'll also need to track a third value, aim, which also starts at 0. 
    // The commands also mean something entirely different than you first thought:

    // down X increases your aim by X units.
    // up X decreases your aim by X units.
    // forward X does two things:
    //     It increases your horizontal position by X units.
    //     It increases your depth by your aim multiplied by X.

    // After following these new instructions, you would have a horizontal position of 15 and a depth of 60. (Multiplying these produces 900.)

    #[test]
    fn first_example_part_two() {
        let contents = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let mut position = Position2{horizontal : 0, depth : 0, aim: 0};
        let answer_day2_part2 = position.parse(contents);
        assert_eq!(15, position.horizontal);
        assert_eq!(60, position.depth);
        assert_eq!(900, answer_day2_part2);
    }


}