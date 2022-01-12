use std::fs;
use std::error::Error;

pub struct Vents {
    pub graph : Vec<Vec<i32>>,
    pub parsed : Vec<Vec<usize>>
}

impl Vents {
    pub fn new() -> Vents {
        Vents{graph: vec![], parsed: vec![]}
    }

    pub fn parse(&mut self, contents: &str) {
        for line in contents.lines() {
            let xys : Vec<&str> = line.split(" -> ").collect();
            let x1y1 : Vec<&str> = xys[0].split(",").collect();
            let x2y2 : Vec<&str> = xys[1].split(",").collect();
            let row : Vec<usize> = vec![
                x1y1[0].parse().unwrap(),
                x1y1[1].parse().unwrap(),
                x2y2[0].parse().unwrap(),
                x2y2[1].parse().unwrap()
            ];
            self.parsed.push(row);
        }
    }

    pub fn filter_out_non_horizontal_and_vertical(&mut self, contents: &str) {
        self.parse(contents);
        let mut filtered: Vec<Vec<usize>> = vec![];
        for row in self.parsed.clone() {
            // only keep where x1 = x2, or y1 = y2
            if row[0] == row[2] || row[1] == row[3] {
                filtered.push(row.to_vec());
            }
        }
        self.parsed = filtered;
    }

    pub fn extents(&mut self) {
        let mut max_width: usize = 0;
        let mut max_height: usize = 0;
        for coord in &self.parsed {
            if coord[0]+1 > max_width {
                max_width = coord[0]+1
            }
            if coord[2]+1 > max_width {
                max_width = coord[2]+1
            }
            if coord[1]+1 > max_height {
                max_height = coord[1]+1
            }
            if coord[1]+1 > max_height {
                max_height = coord[3]+1
            } 
        }
        for _row in 0..max_height {
            let mut new_row : Vec<i32> = vec![];
            for _elements in 0..max_width {
                new_row.push(0);
            }
            self.graph.push(new_row);
        }
    }

    pub fn plot(&mut self) {
        for line in &self.parsed {
            // determine if x's match, or y's match
            // x1 == x2 
            if line[0] == line[2] {
                    if line[1] < line[3]+1 {
                        for index in line[1]..line[3]+1 {
                        self.graph[index][line[0]] += 1;
                    }
                }
                else {
                    for index in line[3]..line[1]+1 {
                        self.graph[index][line[0]] += 1;
                    }
                }
            }
            else {
                if line[0] < line[2]+1 {
                        for index in line[0]..line[2]+1 {
                        self.graph[line[1]][index] += 1;
                    }
                }
                else {
                    for index in line[2]..line[0]+1 {
                        self.graph[line[1]][index] += 1;
                    }
                }
            }
        }
    }

    pub fn sum(&self) -> i32 {
        let mut sum : i32 = 0;
        for row in &self.graph {
            for index in 0..row.len() {
                //println!("graph: {}", row[index]);
                if row[index] > 1 {
                    sum += 1;
                }
            }
        }
        sum
    }

    pub fn part_one(&mut self, contents: &str) -> i32 {
        self.filter_out_non_horizontal_and_vertical(contents);
        self.extents();
        self.plot();
        self.sum()
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day5 part1 answer: {}", Vents::new().part_one(&contents));
    //println!("day4 part2 answer: {}", Bingo::new().part_two(&contents));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_for_horizontal_vertical() {

        let contents = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
989,858 -> 284,858";
        let mut vents = Vents::new();
        vents.filter_out_non_horizontal_and_vertical(&contents);
        assert_eq!(7, vents.parsed.len());
        //0,9 -> 5,9
        assert_eq!(0, vents.parsed[0][0]);
        assert_eq!(9, vents.parsed[0][1]);
        assert_eq!(5, vents.parsed[0][2]);
        assert_eq!(9, vents.parsed[0][3]);
        //9,4 -> 3,4
        assert_eq!(9, vents.parsed[1][0]);
        assert_eq!(4, vents.parsed[1][1]);
        assert_eq!(3, vents.parsed[1][2]);
        assert_eq!(4, vents.parsed[1][3]);
        //2,2 -> 2,1
        assert_eq!(2, vents.parsed[2][0]);
        assert_eq!(2, vents.parsed[2][1]);
        assert_eq!(2, vents.parsed[2][2]);
        assert_eq!(1, vents.parsed[2][3]);
        //7,0 -> 7,4
        assert_eq!(7, vents.parsed[3][0]);
        assert_eq!(0, vents.parsed[3][1]);
        assert_eq!(7, vents.parsed[3][2]);
        assert_eq!(4, vents.parsed[3][3]);
        //0,9 -> 2,9
        assert_eq!(0, vents.parsed[4][0]);
        assert_eq!(9, vents.parsed[4][1]);
        assert_eq!(2, vents.parsed[4][2]);
        assert_eq!(9, vents.parsed[4][3]);
        //3,4 -> 1,4
        assert_eq!(3, vents.parsed[5][0]);
        assert_eq!(4, vents.parsed[5][1]);
        assert_eq!(1, vents.parsed[5][2]);
        assert_eq!(4, vents.parsed[5][3]);
        //989,858 -> 284,858
        assert_eq!(989, vents.parsed[6][0]);
        assert_eq!(858, vents.parsed[6][1]);
        assert_eq!(284, vents.parsed[6][2]);
        assert_eq!(858, vents.parsed[6][3]);
    }

    #[test]
    fn extents_of_graph() {
        let mut vents = Vents::new();
        vents.parsed.push(vec![0,9,5,9]);
        vents.parsed.push(vec![9,4,3,4]);
        vents.parsed.push(vec![2,2,2,1]);
        vents.parsed.push(vec![7,0,7,4]);
        vents.parsed.push(vec![0,9,2,9]);
        vents.parsed.push(vec![3,4,1,4]);
        vents.parsed.push(vec![989,858,284,858]);

        vents.extents();

        assert_eq!(859, vents.graph.len());
        assert_eq!(990, vents.graph[0].len());
        assert_eq!(990, vents.graph[1].len());
        assert_eq!(990, vents.graph[2].len());
        assert_eq!(990, vents.graph[3].len());
        assert_eq!(990, vents.graph[4].len());
        assert_eq!(990, vents.graph[5].len());
        assert_eq!(990, vents.graph[6].len());
        assert_eq!(990, vents.graph[7].len());
        assert_eq!(990, vents.graph[8].len());
    }

    #[test]
    fn can_plot_to_extents() {
        let mut vents = Vents::new();
        vents.parsed.push(vec![0,9,5,9]);
        vents.parsed.push(vec![9,4,3,4]);
        vents.parsed.push(vec![2,2,2,1]);
        vents.parsed.push(vec![7,0,7,4]);
        vents.parsed.push(vec![0,9,2,9]);
        vents.parsed.push(vec![3,4,1,4]);
        vents.parsed.push(vec![989,858,284,858]);

        vents.extents();

        vents.plot();

        assert_eq!(1, vents.graph[858][284]);
        assert_eq!(1, vents.graph[858][989]);
    }

    #[test]
    fn first_example_part_one() {
        let contents = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
9,6 -> 9,9";
        let mut vents = Vents::new();
        let answer = vents.part_one(&contents);
        assert_eq!(5, answer);
    }
}