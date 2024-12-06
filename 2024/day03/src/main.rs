mod part1 {
    use regex::Regex;
    use std::fs::read_to_string;

    pub fn sol() -> i32 {
        let input = read_to_string("input.txt").unwrap();
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        re.captures_iter(&input)
            .map(|c| c.extract())
            .map(|(_, [val1, val2])| val1.parse::<i32>().unwrap() * val2.parse::<i32>().unwrap())
            .into_iter()
            .sum()
    }
}

mod part2 {
    use regex::Regex;
    use std::fs::read_to_string;

    enum Token {
        Mul { a: i32, b: i32 },
        Do,
        Dont,
    }

    pub fn sol() -> i32 {
        let input = read_to_string("input.txt").unwrap();
        let re = Regex::new(r"mul\((\d+),(\d+)\)|(do\(\)()|don't\(\)())").unwrap();
        re.captures_iter(&input)
            .map(|x| x.extract())
            .map(|(_, [a, b])| match (a, b) {
                ("do()", _) => Token::Do,
                ("don't()", _) => Token::Dont,
                (_, _) => Token::Mul {
                    a: a.parse::<i32>().unwrap(),
                    b: b.parse::<i32>().unwrap(),
                },
            })
            .fold((true, 0), |(doing, sum), next| match next {
                Token::Do => (true, sum),
                Token::Dont => (false, sum),
                Token::Mul { a, b } => {
                    if doing {
                        (doing, sum + a * b)
                    } else {
                        (doing, sum)
                    }
                }
            })
            .1
    }
}

fn main() {
    println!("{}", part1::sol());
    println!("{}", part2::sol());
}
