use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day10 part1 answer: {}", SyntaxScoring::new().part_one(&contents));
    //println!("day10 part2 answer: {}", SyntaxScoring::new().part_two(&contents));
    Ok(())
}

pub struct SyntaxScoring {

}

impl SyntaxScoring {
    pub fn new() -> SyntaxScoring {
        SyntaxScoring{}
    }

    pub fn part_one(&self, contents: &str) -> usize {
        // open character.... on stack
        // close character... needs to match last open character
        let mut syntax_error_score: usize = 0;
        for row in contents.lines() 
        {
            let mut opening_characters : Vec<char> = vec![];
            for ch in row.chars() 
            {
                // if opening character add to opening characters
                if ch == '(' || ch == '[' || ch == '{' || ch == '<' 
                {
                    opening_characters.push(ch);
                } 
                else 
                {
                    let last_character = opening_characters[opening_characters.len()-1];
                    opening_characters.pop();
                    if (ch == ')' && last_character != '(') ||
                       (ch == ']' && last_character != '[') ||
                       (ch == '}' && last_character != '{') ||
                       (ch == '>' && last_character != '<')
                    {    
                        if ch == ')' 
                        {
                            syntax_error_score += 3;
                        } 
                        else if ch == ']' 
                        {
                            syntax_error_score += 57;
                        } 
                        else if ch == '}' 
                        {
                            syntax_error_score += 1197;
                        } 
                        else 
                        {
                            syntax_error_score += 25137;
                        }
                        break;
                    }
                }
            }
        }
        syntax_error_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_points() {
        let contents = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(26397, SyntaxScoring::new().part_one(contents));
    }

}