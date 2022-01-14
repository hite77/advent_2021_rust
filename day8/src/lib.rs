use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day8 part1 answer: {}", SevenSegment::new().part_one(&contents));
    //println!("day8 part2 answer: {}", CrabSubmarine::new().part_two(&contents));
    Ok(())
}

pub struct SevenSegment {
    outputs: Vec<usize>
}

impl SevenSegment {
   pub fn new() -> SevenSegment {
       SevenSegment {outputs: vec![]}
    }

    pub fn parse(&mut self, contents: &str) {
        for line in contents.lines() {
            let split_string : Vec<&str> = line.split(" | ").collect();
            for output_string in split_string[1].split(" ") {
                self.outputs.push(output_string.len())
            }
        }
    }

    pub fn part_one(&mut self, contents: &str) -> usize {
        self.parse(contents);
        // count after " | " on each line solut with space
        // 1 --> 2 digits
        // 4 --> 4 digits
        // 7 --> 3 digits
        // 8 --> 7 digits
        let mut count : usize = 0;
        for index in 0..self.outputs.len() {
            let item = self.outputs[index];
            if item == 2 || item == 4 || item == 3 || item == 7 {
                count += 1;
            }
        }
        count
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_1_4_7_8() {
        let mut seven_segment = SevenSegment::new();
                let contents = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        assert_eq!(26, seven_segment.part_one(contents));  
    }
}

// 0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

//4 digits and wires scrambled....

// notes
// acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
// cdfeb fcadb cdfeb cdbaf
// single line

//acf (7) only 3 digit ==>  dab
//bcdf(4) only 4 digit ==?> eafb

//1, 4, 7, or 8 after | 
// 1 --> 2 digits
// 4 --> 4 digits
// 7 --> 3 digits
// 8 --> 7 digits

// 10 signals and then a | and then four digit out value
