//16,1,2,0,4,2,7,1,2,14
// least fuel is position 2

    // Move from 16 to 2: 14 fuel
    // Move from 1 to 2: 1 fuel
    // Move from 2 to 2: 0 fuel
    // Move from 0 to 2: 2 fuel
    // Move from 4 to 2: 2 fuel
    // Move from 2 to 2: 0 fuel
    // Move from 7 to 2: 5 fuel
    // Move from 1 to 2: 1 fuel
    // Move from 2 to 2: 0 fuel
    // Move from 14 to 2: 12 fuel

// total fuel 37
// position 1 (41 fuel), position 3 (39 fuel), position 10 (71 fuel)

// answer is the amount of fuel

use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day7 part1 answer: {}", CrabSubmarine::new().part_one(&contents));
    println!("day7 part2 answer: {}", CrabSubmarine::new().part_two(&contents));
    Ok(())
}

pub struct CrabSubmarine {
	positions : Vec<usize>
}

impl CrabSubmarine {
    pub fn new() -> CrabSubmarine {
        CrabSubmarine {positions: vec![]}
    }

    pub fn parse(&mut self, contents: &str) {
    	for item in contents.split(",") {
    		self.positions.push(item.parse().unwrap());
    	}
    }

    pub fn fuel_used(&self, position: usize) -> usize {
    	let mut fuel_used : usize = 0;
    	for item in &self.positions {
    		if position < *item {
    			fuel_used += item-position;
    		} else {
    			fuel_used += position-item;
    		}
    	}
    	fuel_used
    }

	pub fn fuel_used_more_each_time(&self, position: usize) -> usize {
    	let mut fuel_used : usize = 0;
    	for item in &self.positions {
    		let mut step_size = 1;
    		if position < *item {
    			for _steps in position..*item {
    				fuel_used += step_size;
    				step_size += 1;
    			}
    		} 
    		else 
    		{
    			let end : usize = position;
    			let begin : usize = *item;
    			for _steps in begin..end {
    				fuel_used += step_size;
    				step_size += 1;
    			}
    		}
    	}
    	fuel_used
    }


    fn extents(&self) -> (usize , usize) {
    	let mut low = self.positions[0];
    	let mut high = self.positions[0];
    	for item in &self.positions {
    		if item > &high {
    			high = *item;
    		}
    		if item < &low {
    			low = *item;
    		}
    	}
    	(low, high)
    }

    pub fn part_one(&mut self, contents: &str) -> usize {
    	self.parse(contents);
    	let (start, end) = self.extents();
    	let mut lowest_fuel : usize = self.fuel_used(start);
    	for trial_position in start..end+1 {
    		let current_fuel_used = self.fuel_used(trial_position);
    		if  current_fuel_used < lowest_fuel {
    			lowest_fuel = current_fuel_used;
    		}
    	}
    	lowest_fuel
    }

    pub fn part_two(&mut self, contents: &str) -> usize {
    	self.parse(contents);
    	let (start, end) = self.extents();
    	let mut lowest_fuel : usize = self.fuel_used_more_each_time(start);
    	for trial_position in start..end+1 {
    		let current_fuel_used = self.fuel_used_more_each_time(trial_position);
    		if  current_fuel_used < lowest_fuel {
    			lowest_fuel = current_fuel_used;
    		}
    	}
    	lowest_fuel
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuel_used_calculates_fuel() {
    	let mut crab_submarine = CrabSubmarine::new();
    	crab_submarine.parse("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(37, crab_submarine.fuel_used(2));
     	assert_eq!(41, crab_submarine.fuel_used(1));
     	assert_eq!(39, crab_submarine.fuel_used(3));
        assert_eq!(71, crab_submarine.fuel_used(10));
    }

    #[test]
    fn fuel_used_more_each_time() {
    	let mut crab_submarine = CrabSubmarine::new();
    	crab_submarine.parse("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(168, crab_submarine.fuel_used_more_each_time(5));
        assert_eq!(206, crab_submarine.fuel_used_more_each_time(2));
    }

    #[test]
    fn  both_parts_example_passes() {
    	assert_eq!(37, CrabSubmarine::new().part_one("16,1,2,0,4,2,7,1,2,14"));
	  	assert_eq!(168, CrabSubmarine::new().part_two("16,1,2,0,4,2,7,1,2,14"));
    }
}
