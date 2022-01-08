use std::fs;
use std::error::Error;

pub struct Binary {
  pub gamma:    String,
  pub episolon: String,
}

impl Binary {
    pub fn new() -> Binary {
        Binary { gamma: "".to_string(), episolon: "".to_string()}
    }
    pub fn parse(&mut self, contents: &str) -> i32 {
        let mut sum: Vec<i32> = vec![];
        let mut count: i32 = 0;
        for line in contents.lines() {
            let mut v: Vec<i32> = vec![];
            count += 1;
            for c in line.chars() {
                match c {
                    '0' => v.push(0),
                    '1' => v.push(1),
                     _  => println!("Unmatched!!!!")
                }
            }
            // add to sum, if no length then sum becomes first vector
            if sum.len() > 0 {
                for (i, x) in v.iter().enumerate() {
                    sum[i] += x;
                }
            }
            else {
                sum = v;
            }
        }
        for x in sum {
            if x as f32 / count as f32 > 0.5 {
                self.episolon += "1";
                self.gamma += "0";
            }
            else {
                self.episolon += "0";
                self.gamma += "1";
            }
        }
    

        (isize::from_str_radix(&self.episolon, 2).unwrap() * isize::from_str_radix(&self.gamma, 2).unwrap()).try_into().unwrap()
    }
}

pub struct BinaryPart2 {
  pub ogr:    String,
  pub coscrubber: String,
}

impl BinaryPart2 {
    pub fn new() -> BinaryPart2 {
        BinaryPart2 { ogr: "".to_string(), coscrubber: "".to_string()}
    }

    pub fn find_next_digit_co2(&mut self, contents: &str) -> bool {
        let mut done = false;
        let mut sum: i32 = 0;
        let mut count: i32 = 0; // count of 1 means done at end.

        for line in contents.lines() {
            if line.starts_with(&self.coscrubber) {
                count += 1;
                let start = self.coscrubber.len();
                let end = start+1;
                match line.get(start..end) {
                    Some("1") => sum += 1,
                    Some("0") => sum += 0,
                    _         => println!("Other"),
                }
            }
        }
        
        if sum as f32 / (count as f32) < 0.5 
        {
            self.coscrubber += "1";
        }
        else 
        {
            self.coscrubber += "0";
        }

        count = 0;
        let mut solution = "".to_string();
        for line in contents.lines() {
            if line.starts_with(&self.coscrubber) {
                solution = line.to_string();
                count += 1;
            }
        }

        if count == 1 {
            done = true;
            self.coscrubber = solution.to_string();
        }
        
        done
    }

    pub fn find_next_digit_ogr(&mut self, contents: &str) -> bool {
        let mut done = false;
        let mut sum: i32 = 0;
        let mut count: i32 = 0; // count of 1 means done at end.

        for line in contents.lines() {
            if line.starts_with(&self.ogr) {
                count += 1;
                let start = self.ogr.len();
                let end = start+1;
                match line.get(start..end) {
                    Some("1") => sum += 1,
                    Some("0") => sum += 0,
                    _         => println!("Other"),
                }
            }
        }
        
        if sum as f32 / count as f32 >= 0.5 
        {
            self.ogr += "1";
        }
        else 
        {
            self.ogr += "0";
        }

        count = 0;
        let mut solution = "".to_string();
        for line in contents.lines() {
            if line.starts_with(&self.ogr) {
                solution = line.to_string();
                count += 1;
            }
        }

        if count == 1 {
            done = true;
            self.ogr = solution.to_string();
        }
        
        done
    }

    pub fn parse(&mut self, contents: &str) -> i32 {
        while false == self.find_next_digit_ogr(&contents) {}
    
        while false == self.find_next_digit_co2(&contents) {}
        
        (isize::from_str_radix(&self.ogr, 2).unwrap() * isize::from_str_radix(&self.coscrubber, 2).unwrap()).try_into().unwrap()
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day3 part1 answer: {}", Binary::new().parse(&contents));
    println!("day3 part2 answer: {}", BinaryPart2::new().parse(&contents));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example_part_one() {
        let contents = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let mut binary = Binary::new();
        let answer = binary.parse(&contents);
        assert_eq!("10110", binary.episolon);
        assert_eq!("01001", binary.gamma); 
        assert_eq!(198, answer);
    }

    #[test]
    fn first_example_part_two() {
        let contents = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let mut binary_part2 = BinaryPart2::new();
        let answer = binary_part2.parse(&contents);
        assert_eq!("10111", binary_part2.ogr);
        assert_eq!("01010", binary_part2.coscrubber); 
        assert_eq!(230, answer);
    }

    #[test]
    fn next_digit_works_for_digits_ogr() {
        let contents = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let mut binary_part2 = BinaryPart2::new();
        let finished = binary_part2.find_next_digit_ogr(contents);
        assert_eq!(false, finished);
        assert_eq!("1", binary_part2.ogr);
        let finished = binary_part2.find_next_digit_ogr(contents);
        assert_eq!(false, finished);
        assert_eq!("10", binary_part2.ogr);
        let finished = binary_part2.find_next_digit_ogr(contents);
        assert_eq!(false, finished);
        assert_eq!("101", binary_part2.ogr);
        let finished = binary_part2.find_next_digit_ogr(contents);
        assert_eq!(false, finished);
        assert_eq!("1011", binary_part2.ogr);
        let finished = binary_part2.find_next_digit_ogr(contents);
        assert_eq!(true, finished);
        assert_eq!("10111", binary_part2.ogr);
    }

    #[test]
    fn next_digit_works_for_digits_co2() {
        let contents = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
        let mut binary_part2 = BinaryPart2::new();
        let finished = binary_part2.find_next_digit_co2(contents);
        assert_eq!(false, finished);
        assert_eq!("0", binary_part2.coscrubber);
        let finished = binary_part2.find_next_digit_co2(contents);
        assert_eq!(false, finished);
        assert_eq!("01", binary_part2.coscrubber);
        let finished = binary_part2.find_next_digit_co2(contents);
        assert_eq!(true, finished);
        assert_eq!("01010", binary_part2.coscrubber);
    }
}