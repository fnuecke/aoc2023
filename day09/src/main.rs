use itertools::Itertools;

fn parse_sequence(input: &str) -> Vec<i64> {
    input
        .trim()
        .split_whitespace()
        .map(|it| it.parse().unwrap())
        .collect()
}

fn build_derivatives(sequence: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut pyramid = Vec::new();
    pyramid.push(sequence.clone());
    while pyramid.last().unwrap().iter().any(|it| *it != 0) {
        pyramid.push(
            pyramid
                .last()
                .unwrap()
                .iter()
                .tuple_windows()
                .map(|(a, b)| *b - *a)
                .collect(),
        );
    }
    pyramid
}

fn extend_sequence(pyramid: &mut Vec<Vec<i64>>) {
    pyramid.last_mut().unwrap().push(0);
    for index in (0..pyramid.len() - 1).rev() {
        let next = pyramid[index].last().unwrap() + pyramid[index + 1].last().unwrap();
        pyramid[index].push(next);
    }
}

fn solve(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut pyramid = build_derivatives(&parse_sequence(line));
        extend_sequence(&mut pyramid);
        sum += pyramid[0].last().unwrap();
    }
    return sum;
}

fn solve2(input: &str) -> i64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut sequence = parse_sequence(line);
        sequence.reverse();
        let mut pyramid = build_derivatives(&sequence);
        extend_sequence(&mut pyramid);
        sum += pyramid[0].last().unwrap();
    }
    return sum;
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let output = solve(input.as_str());
    println!("{output}");

    let output = solve2(input.as_str());
    println!("{output}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";

    #[test]
    fn sample() {
        let expected_output = 114;

        let output = solve(TEST_INPUT);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample2() {
        let expected_output = 2;

        let output = solve2(TEST_INPUT);

        assert_eq!(expected_output, output);
    }
}
