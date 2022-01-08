use std::fs;
use std::error::Error;

pub struct Binary {
  pub gamma:    String,
  pub episolon: String,
}

impl Binary {
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
        // 1 / 3 < 0.5 then 0
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

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    let mut binary = Binary{episolon:"".to_string(), gamma:"".to_string()};
    println!("day3 part1 answer: {}", binary.parse(&contents));
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
        let mut binary = Binary{episolon:"".to_string(), gamma:"".to_string()};
        let answer = binary.parse(contents);
        assert_eq!("10110", binary.episolon);
        assert_eq!("01001", binary.gamma); 
        assert_eq!(198, answer);
    }
}