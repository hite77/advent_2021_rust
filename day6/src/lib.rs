use std::fs;
use std::error::Error;

pub struct Lanternfish {

}

impl Lanternfish {
    pub fn new() -> Lanternfish {
        Lanternfish {}
    }

    pub fn parse(&self, input : &str) -> Vec<usize> {
        let mut vector : Vec<usize> = vec![0,0,0,0,0,0,0,0,0];
        // 0,1,2,3,4,5,6,7,8
        for item in input.split(",") {
            let index: usize = item.parse().unwrap();
            vector[index] += 1
        }
        vector
    }

    pub fn part_one(&self, input : &str, days: usize) -> usize
    {
        let mut current_row : Vec<usize> = self.parse(input);
        for _days in 0..days {
                                             // 0,1,2,3,4,5,6,7,8
            let mut new_row : Vec<usize> = vec![0,0,0,0,0,0,0,0,0];
            for index in 0..9 {
                if index == 0 {
                    //becomes 8 and 6
                    new_row[8] = current_row[0];
                    new_row[6] = current_row[0];
                } else {
                    // N becomes N-1
                    new_row[index-1] += current_row[index]
                }
            }
            current_row = new_row.clone();
            //println!("pretty print current_row:{:#?}", current_row);
        }
        let mut total: usize = 0;
        for item in current_row {
            total += item;
        }
        total
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day6 part1 answer: {}", Lanternfish::new().part_one(&contents, 80));
    println!("day6 part2 answer: {}", Lanternfish::new().part_one(&contents, 256));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_gets_vector() {
        let actual : Vec<usize> = Lanternfish::new().parse("3,2,3,1,2");
        // 0,1,2,3,4,5,6,7,8
        // 0,1,2,2,0,0,0,0,0
        let expected : Vec<usize> = vec![0,1,2,2,0,0,0,0,0];
        assert_eq!(expected, actual);
        let another_expected : Vec<usize> = vec![0,0,0,0,0,0,0,0,3];
        assert_eq!(another_expected, Lanternfish::new().parse("8,8,8"));
    }

    #[test]
    fn count_the_lanternfish() {

        let input = "\
3,4,3,1,2";
        assert_eq!(26, Lanternfish::new().part_one(input, 18));
        assert_eq!(5934, Lanternfish::new().part_one(input, 80));
        assert_eq!(26984457539, Lanternfish::new().part_one(input, 256));
    }
}
