use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    //1640 was too high
    println!("day9 part1 answer: {}", LavaTubes::new().part_one(&contents));
    println!("day9 part2 answer: {}", LavaTubes::new().part_two(&contents));
    Ok(())
}

struct Coord {
    pub x: usize, pub y: usize
}

pub struct LavaTubes {
    grid: Vec<Vec<usize>>,
    coords: Vec<Coord>,
    basin_sizes: Vec<usize>
}

impl LavaTubes {
    pub fn new() -> LavaTubes {
        LavaTubes{grid: vec![], coords: vec![], basin_sizes: vec![]}
    }

// 2199943210
// 3987894921
// 9856789892
// 8767896789
// 9899965678

    pub fn part_two(&mut self, contents: &str) -> usize {
       self.parse(contents);
       for row_index in 0..self.grid.len() {
            for column_index in 0..self.grid[0].len() {
                if self.check_around(row_index, column_index) {
                    // find basin size....
                    self.coords = vec![];
                    let basin_size = self.spelunk(row_index, column_index);
                    if self.basin_sizes.len() < 3 {
                        self.basin_sizes.push(basin_size);
                    } 
                    else if basin_size > self.basin_sizes[0] || 
                            basin_size > self.basin_sizes[1] ||
                            basin_size > self.basin_sizes[2]
                    {
                        // find lowest value of the three.
                        let mut lowest_value = self.basin_sizes[0];
                        let mut index : usize = 0;
                        if self.basin_sizes[1] < lowest_value {
                            lowest_value = self.basin_sizes[1];
                            index = 1;
                        }
                        if self.basin_sizes[2] < lowest_value {
                            index = 2;
                        }
                        self.basin_sizes[index] = basin_size;
                    }
                }
            }
            // need three biggest times...
        }
        self.basin_sizes[0] * self.basin_sizes[1] * self.basin_sizes[2]
    }

    pub fn not_in(&self, row: usize, column: usize) -> bool {
        for coord in &self.coords {
            if coord.x == row && coord.y == column {
                return false;
            }
        }
        true
    }

    // function called from lowest point.... 
    // 1 for it...
    // record my coord....
    // call function in directions that are valid and aren't in coords
    // and not 9's

    pub fn spelunk(&mut self, row: usize, column: usize) -> usize {
        let mut size : usize = 1;
        let coord: Coord = Coord {x: row, y: column };
        self.coords.push(coord);

        // call for non 9 values and for ones that are valid and not in coords
        let next_row : i32 = row.try_into().unwrap();
        let next_col : i32 = column.try_into().unwrap();

        if next_row-1 >= 0 {
            if self.grid[row-1][column] != 9 && self.not_in(row-1, column) {
                size += self.spelunk(row-1, column)
            }
        }
        if row+1 <= self.grid.len()-1 {
            if self.grid[row+1][column] != 9 && self.not_in(row+1, column) {
                size += self.spelunk(row+1, column)
            }
        }
        if next_col-1 >= 0 {
            if self.grid[row][column-1] != 9 && self.not_in(row, column-1) {
                size += self.spelunk(row, column-1);
            }
        }
        if column+1 <= self.grid[0].len()-1 {
            if self.grid[row][column+1] != 9 && self.not_in(row, column+1) {
                size += self.spelunk(row, column+1);
            }
        }
        size
    }

    pub fn parse(&mut self, contents: &str) {
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
    }

    pub fn part_one(&mut self, contents: &str) -> usize {
        self.parse(contents);
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

    #[test]
    fn equal_doesnt_count() {
let contents = "\
111
111
111";
        assert_eq!(0, LavaTubes::new().part_one(contents));
    }

    #[test]
    fn three_largest_basins_multiplied() {
        assert_eq!(1134, LavaTubes::new().part_two(CONTENTS));
    }
}