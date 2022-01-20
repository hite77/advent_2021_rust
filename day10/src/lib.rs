use std::fs;
use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("input")?;
    println!("day10 part1 answer: {}", SyntaxScoring::new().part_one(&contents));
    println!("day10 part2 answer: {}", SyntaxScoring::new().part_two(&contents));
    Ok(())
}

pub struct SyntaxScoring {

}

impl SyntaxScoring {
    pub fn new() -> SyntaxScoring {
        SyntaxScoring{}
    }

    // drop lines that are broken....
    // complete line and track characters
    // use the scoring system to get points
    // sort points and return middle score
    pub fn part_two(&self, contents: &str) -> usize {
        let mut scores : Vec<usize> = vec![];
        for row in contents.lines() 
        {
            let (score, opening_characters) = self.score_for_broken_line(row);
            if score == 0 {
                //row ends with characters left off
                let mut score: usize = 0;
                for item in opening_characters.iter().rev() {
                    score *= 5;
                    if *item == '(' 
                    {
                        score += 1;
                    }
                    else if *item == '['
                    {
                        score += 2;
                    }
                    else if *item == '{'
                    {
                        score += 3;
                    }
                    else 
                    {
                        score += 4;
                    }
                }
                scores.push(score);
            }
        }
        self.sort_and_return_middle(scores)
    }

    pub fn sort_and_return_middle(&self, mut scores: Vec<usize>) -> usize {
        //sort
        let mut swap: bool = true;
        while swap {
            swap = false;
            for index in 0..scores.len()-1 {
                if scores[index+1] < scores[index] {
                    swap = true;
                    let swapper: usize = scores[index];
                    scores[index] = scores[index+1];
                    scores[index+1] = swapper;
                }
            }    
        }
        scores[(scores.len()+1)/2-1]
    }

    pub fn score_for_broken_line(&self, row: &str) -> (usize,  Vec<char>) {
        // open character.... on stack
        // close character... needs to match last open character
        let mut opening_characters : Vec<char> = vec![];
        let mut syntax_error_score: usize = 0;
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
        (syntax_error_score , opening_characters)
    }

    pub fn part_one(&self, contents: &str) -> usize {
        let mut syntax_error_score: usize = 0;
        for row in contents.lines() 
        {
            let (current_score, _unneeded) = self.score_for_broken_line(row);
            syntax_error_score += current_score;
        }
        syntax_error_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_example() {
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

    #[test]
    fn part_two_example
    () {
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
        assert_eq!(288957, SyntaxScoring::new().part_two(contents));
    }

}