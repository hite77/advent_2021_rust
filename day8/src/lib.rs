use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day8 part1 answer: {}", SevenSegment::new().part_one(&contents));
    println!("day8 part2 answer: {}", SevenSegment::new().part_two(&contents));
    Ok(())
}

pub struct SevenSegment {
    outputs: Vec<usize>,
    left: Vec<String>,
    right: Vec<String>
}

impl SevenSegment {
   pub fn new() -> SevenSegment {
       SevenSegment {outputs: vec![], left: vec![], right: vec![]}
    }

    pub fn parse(&mut self, contents: &str) {
        for line in contents.lines() {
            let split_string : Vec<&str> = line.split(" | ").collect();
            self.left.push(split_string[0].to_string());
            self.right.push(split_string[1].to_string());
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

    pub fn part_two(&mut self, contents: &str) -> usize {
        // parse splits into left and right....
        self.parse(contents);
        let mut grand_total : usize = 0;
        // need left side -- to decode.....
        // need right side + a->g equal to get totals and sum.
        for index in 0..self.left.len() {
            // split and find 1, 4, 7, 8 (will always be abcdefg)
            // namely 5,6,2,4,3,7
            let a: String;
            let mut b: String;
            let mut c: String;
            let mut d: String;
            let mut e: String;
            let mut f: String;
            let mut g: String;

            let mut five : Vec<String> = vec![];
            let mut six: Vec<String> = vec![];
            let mut two: Vec<String> = vec![];
            let mut four: Vec<String> = vec![];
            let mut three: Vec<String> = vec![];
            let mut seven: Vec<String> = vec![];

            for item in self.left[index].split(" ") {
                // namely 5,6,2,4,3,7
                match item.len() {
                    5 => five.push(item.to_string()),
                    6 => six.push(item.to_string()),
                    2 => two.push(item.to_string()),
                    4 => four.push(item.to_string()),
                    3 => three.push(item.to_string()),
                    7 => seven.push(item.to_string()),
                    _ => println!("Never called"),
                }
            } // finishing split
            c = two[0].clone(); 
            f = c.clone(); 
            b = self.remove(four[0].clone(), c.clone());
            d = b.clone();
            a = self.remove(three[0].clone(), c.clone());
            e = self.remove("abcdefg".to_string(), a.clone()+&b+&c);
            g = e.clone();

            for item in &six {
                for ch in c.clone().chars() {
                    if item.contains(ch) == false {
                        c = ch.to_string();
                        f = self.remove(f, c.clone());
                        break;
                    }
                }
            }

            for item in five {
                for ch in b.clone().chars() {
                    if item.contains(ch) == false {
                        b= ch.to_string();
                        d= self.remove(d, b.clone());
                        break;
                    }
                }
            }

            for item in six {
                for ch in e.clone().chars() {
                    if item.contains(ch) == false {
                        e= ch.to_string();
                        g= self.remove(g, e.clone());
                        break;
                    }
                }
            }

            let mut digits_encoded : Vec<String> = vec![];
            for item in self.right[index].split(" ") {
                digits_encoded.push(item.to_string());
            }

            let v = vec![a,b,c,d,e,f,g];

            let output_value = self.convert(v.clone(), digits_encoded[0].clone())*1000+
                               self.convert(v.clone(), digits_encoded[1].clone())*100+ 
                               self.convert(v.clone(), digits_encoded[2].clone())*10+
                               self.convert(v.clone(), digits_encoded[3].clone());

            grand_total += output_value;
        } // finished line

        grand_total
    }

    pub fn convert(&self, a_g: Vec<String>, digit_encoded: String) -> usize {
        let a = &a_g[0];
        let b = &a_g[1];
        let c = &a_g[2];
        let d = &a_g[3];
        let e = &a_g[4];
        let f = &a_g[5];
        let g = &a_g[6];
        
        if self.matches(digit_encoded.clone(), a.to_owned()+b+c+e+f+g) {
            return 0;
        }
        if self.matches(digit_encoded.clone(), c.to_owned()+f) {
            return 1;
        }
        if self.matches(digit_encoded.clone(), a.to_owned()+c+d+e+g) {
            return 2;
        }
        if self.matches(digit_encoded.clone(), a.to_owned()+c+d+f+g) {
            return 3;
        }
        if self.matches(digit_encoded.clone(), b.to_owned()+c+d+f) {
            return 4;
        }
        if self.matches(digit_encoded.clone(), a.to_owned()+b+d+f+g) {
            return 5;
        }
        if self.matches(digit_encoded.clone(), a.to_owned()+b+d+e+f+g) {
            return 6;
        }
        if self.matches(digit_encoded.clone(), a.to_owned()+c+f) {
            return 7;
        }
        if self.matches(digit_encoded.clone(), a.to_owned()+b+c+d+e+f+g) {
            return 8;
        }
        9
    }

    pub fn matches(&self, source: String, compare: String) -> bool {
        if source.len() != compare.len() {
            return false;
        }
        for c in source.chars() {
            if false == compare.contains(c) {
                return false;
            }
        }
        true
    }

    pub fn remove(&self, source: String, remove: String) -> String {
        let mut built_string : String = "".to_string();
        for c in source.chars() {
            if false == remove.contains(c) {
                built_string.push(c);
            }
        }
        built_string
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    static CONTENTS: &str = "\
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

    #[test]
    fn count_1_4_7_8() {
        let mut seven_segment = SevenSegment::new();
        assert_eq!(26, seven_segment.part_one(CONTENTS));
    }

    #[test]
    fn matches_examples() {
        assert_eq!(true, SevenSegment::new().matches("foo".to_string(), "ofo".to_string()));
        assert_eq!(false, SevenSegment::new().matches("foo".to_string(), "bar".to_string()));
        assert_eq!(false, SevenSegment::new().matches("foo".to_string(), "food".to_string()));
    }

    #[test]
    fn parse_two_small_example() {
        let answer = SevenSegment::new().part_two("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
        assert_eq!(5353, answer);
    }

    #[test]
    fn part_two_entire_example() {
        let mut seven_segment = SevenSegment::new();
        assert_eq!(61229, seven_segment.part_two(CONTENTS));  
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

//   -- digits lit
// 0 -- 6
// 1 -- 2*
// 2 -- 5
// 3 -- 5
// 4 -- 4*
// 5 -- 5
// 6 -- 6
// 7 -- 3*
// 8 -- 7*
// 9 -- 6

// mappings
// 0 -- abcefg
// 1 -- cf
// 2 -- acdeg
// 3 -- acdfg
// 4 -- bcdf
// 5 -- abdfg
// 6 -- abdefg
// 7 -- acf
// 8 -- abcdefg
// 9 -- abcdfg

// each line will need to decode.
// mixed mapping
// 1 -- ab
// 4 -- eafb
// 7 -- dab
// 8 -- acedgfb
// above are based on digits
//




// find the 1, 4, 7, 8 numbers.....

//acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab 
//| cdfeb fcadb cdfeb cdbaf

// hash map --> eventually
//acedgfb -- 8
//cdfbe(5)
//gcdfa(5)
//fbcad(5)
//dab -- 7
//cefabd(6)
//cdfgeb(6)
//eafb -- 4
//cagedb(6)
//ab -- 1

// using digits..... to find 1, 4, 7, 8
// between 1 and 4... you can find what is possible.... 


// in progress....


//  dddd
// e    a
// e    a
//  ffff
// g    b
// g    b
//  cccc


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
