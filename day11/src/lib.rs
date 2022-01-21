use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day11 part1 answer: {}", DumboOctopus::new().part_one(&contents));
    //println!("day11 part2 answer: {}", DumboOctopus::new().part_two(&contents));
    Ok(())
}

pub struct DumboOctopus {
    pub grid: Vec<Vec<usize>>,
    pub flash: Vec<Vec<usize>>
}

impl DumboOctopus {
    pub fn new() -> DumboOctopus {
        DumboOctopus{grid: vec![], flash: vec![]}
    }

    pub fn parse(&mut self, contents: &str) {
        for row in contents.lines() {
            let mut new_row: Vec<usize> = vec![];
            let mut flash_row: Vec<usize> = vec![];
            for ch in row.chars() {
                flash_row.push(0);
                match ch {
                    '0' => new_row.push(0),
                    '1' => new_row.push(1),
                    '2' => new_row.push(2),
                    '3' => new_row.push(3),
                    '4' => new_row.push(4),
                    '5' => new_row.push(5),
                    '6' => new_row.push(6),
                    '7' => new_row.push(7),
                    '8' => new_row.push(8),
                    '9' => new_row.push(9),
                     _  => println!("not called"),
                }
            }
            self.grid.push(new_row);
            self.flash.push(flash_row);
        }
    }

    pub fn part_one(&mut self, contents: &str) -> usize {
        println!("inside part one");
        self.parse(contents);
        self.steps(100)
    }

    pub fn update_in_grid(&mut self, row:i32, column: i32) {
        if row >= 0 && row < self.grid.len().try_into().unwrap() {
            if column >= 0 && column < self.grid[0].len().try_into().unwrap() {
                let urow : usize = row.try_into().unwrap();
                let ucolumn : usize = column.try_into().unwrap();
                self.grid[urow][ucolumn] = self.grid[urow][ucolumn] + 1;
                if self.grid[urow][ucolumn] > 9 && self.flash[urow][ucolumn] == 0 {
                    self.flash[urow][ucolumn] = 1;
                    self.call_all_around(row, column);
                } 
            }
        }
    }

    pub fn call_all_around(&mut self, row: i32, column: i32) {
        self.update_in_grid(row-1,column-1);
        self.update_in_grid(row-1, column);
        self.update_in_grid(row-1, column+1);
        self.update_in_grid(row, column-1);
        self.update_in_grid(row, column+1);
        self.update_in_grid(row+1, column-1);
        self.update_in_grid(row+1, column);
        self.update_in_grid(row+1, column+1);
    }

    pub fn steps(&mut self, count: usize) -> usize {
        let mut completed: usize = 0;
        let mut flashes: usize = 0;
        while completed != count {
            // increment by 1.....
            for row_index in 0..self.grid.len() {
                for item_index in 0..self.grid[0].len() {
                    self.grid[row_index][item_index] = self.grid[row_index][item_index] + 1;
                    if self.grid[row_index][item_index] > 9 && self.flash[row_index][item_index] == 0 {
                        self.flash[row_index][item_index] = 1;
                        let irow : i32 = row_index.try_into().unwrap();
                        let icolumn : i32 = item_index.try_into().unwrap();
                        self.call_all_around(irow, icolumn);
                    }
                }
            }

            // clear higher than 9's to zero
            for row_index in 0..self.grid.len() {
                for item_index in 0..self.grid[0].len() {
                    if self.grid[row_index][item_index] > 9 {
                        self.grid[row_index][item_index] = 0;
                    }
                    if self.flash[row_index][item_index] == 1 {
                        flashes += 1;
                        self.flash[row_index][item_index] = 0;
                    }
                }
            }
            println!("completed: {} grid {:?}", completed, self.grid);
            completed += 1;
        }
        flashes
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn smaller_example() {
        let contents = "\
11111
19991
19191
19991
11111";

        let mut dumbo_octopus = DumboOctopus::new();
        dumbo_octopus.parse(contents);
        assert_eq!(9, dumbo_octopus.steps(1));
        dumbo_octopus = DumboOctopus::new();
        dumbo_octopus.parse(contents);
        assert_eq!(9, dumbo_octopus.steps(2));
    }

    #[test]
    fn part_one_example() {
        let contents = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";
        println!("part one full test");
        assert_eq!(1656, DumboOctopus::new().part_one(contents));
    }
}