use std::fs;
use std::error::Error;


pub struct Bingo {
	pub unmarked : i32,
	pub lastcalled: i32
}

impl Bingo {
	pub fn new() -> Bingo {
		Bingo {unmarked: 0, lastcalled: 0}
	}

    pub fn total_solution(&self, board: Vec<Vec<i32>>) -> i32 {
        let mut result : i32 = 0;
        for row in 0..5 {
            for item in 0..5 {
                if board[row][item] != -1 {
                    result += board[row][item]
                }
            }
        }
        result
    }

    pub fn mark_off_called(&self, boards: Vec<Vec<Vec<i32>>>, mark_off: i32) -> Vec<Vec<Vec<i32>>> {
        let mut work_copy = boards.clone();
        for board in 0..work_copy.len() {
            for row in 0..5 {
                for item in 0..5 {
                    if work_copy[board][row][item] == mark_off {
                        work_copy[board][row][item] = -1;
                    }
                }
            }
        }
        work_copy
    }

    pub fn detect_bingo(&self, boards: Vec<Vec<Vec<i32>>>) -> (bool, Vec<Vec<i32>>) {
        let mut result : bool = false;
        let mut solution_board: Vec<Vec<i32>> = vec![];

        for board in 0..boards.len() {
            // horizontal bingo
            for row in 0..5 {
                result = true;
                for item in 0..5 {
                    if boards[board][row][item] != -1 {
                        result = false;
                        break;
                    }
                }
                if result {
                    solution_board = boards[board].clone();
                    return (result, solution_board)
                }
            }
            // vertical bingo
            for item in 0..5 {
                result = true;
                for row in 0..5 {
                    if boards[board][row][item] != -1 {
                        result = false;
                        break;
                    }
                }
                if result {
                    solution_board = boards[board].clone();
                    return (result, solution_board)
                }
            }
        }
        (result, solution_board)
    }

	pub fn part_one(&mut self, contents: &str) -> i32 {
        let mut boards: Vec<Vec<Vec<i32>>> = vec![];
        let mut board: Vec<Vec<i32>> = vec![];
        let mut count: i32 = 0;

		let mut called_numbers : Vec<i32> = vec![];

        for line in contents.lines() {
            if line.contains(",") {
                for item in line.split(",") {
                    called_numbers.push(item.parse().unwrap());
                }
            }
            else if line != "" {
                if count == 0 {
                    board = vec![];
                    count += 1;
                }    
                let mut row: Vec<i32> = vec![];
                for item in line.split(" ") {
                    if item != "" {
                        row.push(item.parse().unwrap());
                    }
                }
                count += 1;
                board.push(row);
                if count > 5 {
                    boards.push(board.clone());
                    count = 0;
                }
            }
	   }
        for item in called_numbers {
           boards = self.mark_off_called(boards, item); 
           let (bingo_happened, solution_board) = self.detect_bingo(boards.clone());
           if bingo_happened {
                self.lastcalled = item;
                self.unmarked = self.total_solution(solution_board);
                return self.lastcalled * self.unmarked;
           }
        }
		self.lastcalled * self.unmarked
	}

}

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day4 part1 answer: {}", Bingo::new().part_one(&contents));
    //println!("day3 part2 answer: {}", BinaryPart2::new().parse(&contents));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_off_called() {
         let mut boards: Vec<Vec<Vec<i32>>> = vec![];
         let mut board: Vec<Vec<i32>> = vec![];
         board.push(vec![22,13,17,11,0]);
         board.push(vec![8,2,23,4,24]);
         board.push(vec![21,9,14,16,7]);
         board.push(vec![6,10,3,18,5]);
         board.push(vec![1,13,20,15,19]);
         boards.push(board);
         let mut board: Vec<Vec<i32>> = vec![];
         board.push(vec![-1,-1,-1,-1,-1]);
         board.push(vec![10,16,15,-1,19]);
         board.push(vec![18,8,-1,26,20]);
         board.push(vec![22,-1,13,6,-1]);
         board.push(vec![-1,-1,12,3,-1]);
         boards.push(board.clone());

         let bingo = Bingo::new();

         boards = bingo.mark_off_called(boards, 19);
         boards = bingo.mark_off_called(boards, 22);

         assert_eq!(-1, boards[0][4][4]);
         assert_eq!(-1, boards[1][1][4]);
         assert_eq!(-1, boards[0][0][0]);
         assert_eq!(-1, boards[1][3][0]);
    }

    #[test]
    fn total_solution_test() {
         let mut board: Vec<Vec<i32>> = vec![];
         board.push(vec![-1,-1,-1,-1,-1]);
         board.push(vec![10,16,15,-1,19]);
         board.push(vec![18,8,-1,26,20]);
         board.push(vec![22,-1,13,6,-1]);
         board.push(vec![-1,-1,12,3,-1]);
     
         let bingo = Bingo::new();

         let answer = bingo.total_solution(board);
         assert_eq!(188, answer);
    }

    #[test]
    fn its_a_bingo() {
        let mut boards: Vec<Vec<Vec<i32>>> = vec![];
        let mut board: Vec<Vec<i32>> = vec![];
         board.push(vec![22,13,17,11,0]);
         board.push(vec![8,2,23,4,24]);
         board.push(vec![21,9,14,16,7]);
         board.push(vec![6,10,3,18,5]);
         board.push(vec![1,13,20,15,19]);
         boards.push(board);
         let mut board: Vec<Vec<i32>> = vec![];
         board.push(vec![-1,-1,-1,-1,-1]);
         board.push(vec![10,16,15,-1,19]);
         board.push(vec![18,8,-1,26,20]);
         board.push(vec![22,-1,13,6,-1]);
         board.push(vec![-1,-1,12,3,-1]);
         boards.push(board.clone());

         let bingo = Bingo::new();

         let (bingo_happened, solution_board) = bingo.detect_bingo(boards.clone());

         assert_eq!(true, bingo_happened);
         assert_eq!(board, solution_board);

         // change to no solution
         boards[1][0][0] = 42;

         let empty_board: Vec<Vec<i32>> = vec![];

         let (bingo_happened, solution_board) = bingo.detect_bingo(boards.clone());
         assert_eq!(false, bingo_happened);
         assert_eq!(empty_board, solution_board);

         // change to vertical solution middle
         boards[0][0][2] = -1;
         boards[0][1][2] = -1;
         boards[0][2][2] = -1;
         boards[0][3][2] = -1;
         boards[0][4][2] = -1;

         let (bingo_happened, solution_board) = bingo.detect_bingo(boards.clone());
         assert_eq!(true, bingo_happened);
         assert_eq!(boards[0], solution_board);
    }

    // add up totals in bingo

    #[test]
    fn first_example_part_one() {
        let contents = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
        let mut bingo = Bingo::new();
        let answer = bingo.part_one(&contents);
        assert_eq!(188, bingo.unmarked);
        assert_eq!(24, bingo.lastcalled); 
        assert_eq!(4512, answer);
    }
}
