use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    //1640 was too high
    println!("day9 part1 answer: {}", LavaTubes::new().part_one(&contents));
    // println!("day9 part2 answer: {}", LavaTubes::new().part_two(&contents));
    Ok(())
}

pub struct LavaTubes {
    grid: Vec<Vec<usize>>
}

impl LavaTubes {
    pub fn new() -> LavaTubes {
        LavaTubes{grid: vec![]}
    }

// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678

    pub fn part_one(&mut self, contents: &str) -> usize {
        for row in contents.lines() {
            let mut row_vector : Vec<usize> = vec![];
            for item in row.chars() {
                match item {
                    '0' => row_vector.push(0),
                    '1' => row_vector.push(1),
                    '2' => row_vector.push(2),
                    '3' => row_vector.push(3),
                    '4' => row_vector.push(4),
                    '5' => row_vector.push(5),
                    '6' => row_vector.push(6),
                    '7' => row_vector.push(7),
                    '8' => row_vector.push(8),
                    '9' => row_vector.push(9),
                     _  => println!("something wrong"),
                }
            }
            self.grid.push(row_vector);
        }
        let mut total: usize = 0;
        for row_index in 0..self.grid.len() {
            for column_index in 0..self.grid[0].len() {
                if self.check_around(row_index, column_index) {
                    total += self.grid[row_index][column_index]+1;
                }
            }
        }
        total
    }

    fn check_around(&self, row: usize, column: usize) -> bool {
        
        let local_row : i32 = row.try_into().unwrap();
        if local_row-1 >= 0 {
            if self.grid[row][column] >= self.grid[row-1][column] {
                return false;
            }
        }
        if row+1 <= self.grid.len()-1 {
            if self.grid[row][column] >= self.grid[row+1][column] {
                return false;
            }
        }
        let local_column: i32 = column.try_into().unwrap();
        if local_column-1 >= 0 {
            if self.grid[row][column] >= self.grid[row][column-1] {
                return false;
            }   
        }
        if column+1 <= self.grid[0].len()-1 {
            if self.grid[row][column] >= self.grid[row][column+1] {
                return false;
            }
        }
        //println!("position: [{}][{}] value:  {}", row, column);
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

// find lowest points --> 1 of 219 and 0 on upper right, 
// 5 3rd row down of 98567
// 5 on bottom row of 5678....

// then add 1 to each of these....

// scan across.... 2 cannot go up, cannot go left... right lower end
// 1(2) cannot go up, left right and down are higher --> mark it...


    static CONTENTS: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn lowest_points() {
        assert_eq!(15, LavaTubes::new().part_one(CONTENTS));
    }
}