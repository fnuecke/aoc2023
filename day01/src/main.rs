fn parse_line(line: &str) -> i32 {
    let first = line.chars().find(|c| char::is_numeric(*c)).unwrap();
    let last = line.chars().rfind(|c| char::is_numeric(*c)).unwrap();
    let value = format!("{first}{last}");
    return value.parse().unwrap();
}

fn parse_lines(input: &str) -> Vec<i32> {
    return input.lines().map(parse_line).collect();
}

fn solve(input: &str) -> i32 {
    return parse_lines(input).iter().sum();
}

const DIGITS: [(&'static str, i32); 20] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("zero", 0),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_line2(line: &str) -> i32 {
    let mut matches = vec![];
    for (c, i) in DIGITS {
        if let Some(index) = line.find(c) {
            matches.push((index, i))
        }
        if let Some(index) = line.rfind(c) {
            matches.push((index, i))
        }
    }
    matches.sort_by_key(|(index, _)| *index);

    let first = matches.first().unwrap().1;
    let last = matches.last().unwrap().1;

    return first * 10 + last;
}

fn parse_lines2(input: &str) -> Vec<i32> {
    return input.lines().map(parse_line2).collect();
}

fn solve2(input: &str) -> i32 {
    return parse_lines2(input).iter().sum();
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let output = solve(input.as_str());
    println!("{output}");

    let output2 = solve2(input.as_str());
    println!("{output2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_output() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let expected_output = 142;

        let output = solve(input);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample_output2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let expected_output = 281;

        let output = solve2(input);

        assert_eq!(expected_output, output);
    }
}
