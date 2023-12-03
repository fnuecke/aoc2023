use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    Red,
    Green,
    Blue,
}

struct Match {
    id: u32,
    sets: Vec<HashMap<Color, u32>>,
}

fn parse_count_color(input: &str) -> (Color, u32) {
    let parts: Vec<&str> = input.trim().split(" ").collect();
    let count: u32 = parts[0].parse().unwrap();
    let color = match parts[1] {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => panic!(),
    };
    return (color, count);
}

fn parse_set(input: &str) -> HashMap<Color, u32> {
    let mut result: HashMap<Color, u32> = HashMap::new();
    for (color, count) in input.split(",").map(parse_count_color) {
        result.insert(color, count);
    }
    return result;
}

fn parse_match(line: &str) -> Match {
    let game_sets: Vec<&str> = line.split(":").collect();
    let game = game_sets[0];
    let sets = game_sets[1];
    let id: u32 = game.trim()[5..].parse().unwrap();
    let sets: Vec<HashMap<Color, u32>> = sets.split(";").map(parse_set).collect();
    return Match { id, sets };
}

fn parse_matches(input: &str) -> Vec<Match> {
    return input.lines().map(parse_match).collect();
}

fn are_sets_valid(sets: &Vec<HashMap<Color, u32>>, limit: &HashMap<Color, u32>) -> bool {
    for set in sets {
        for (color, count) in set {
            if !limit.contains_key(&color) || limit[color] < *count {
                return false;
            }
        }
    }
    return true;
}

fn sum_valid_match_ids(input: Vec<Match>, limit: HashMap<Color, u32>) -> u32 {
    let mut sum = 0;
    for game in input.iter() {
        if are_sets_valid(&game.sets, &limit) {
            sum += game.id;
        }
    }
    return sum;
}

fn game_power(input: &Match) -> u32 {
    let mut maxima: HashMap<Color, u32> = HashMap::new();
    for set in input.sets.iter() {
        for (color, count) in set {
            let max = match maxima.get(color) {
                Some(value) => *count.max(value),
                _ => *count,
            };
            maxima.insert(*color, max);
        }
    }
    return maxima.into_values().reduce(|a, b| a * b).unwrap();
}

fn sum_powers(input: Vec<Match>) -> u32 {
    return input.iter().map(game_power).sum();
}

fn solve(input: &str, limit: HashMap<Color, u32>) -> u32 {
    let matches = parse_matches(input);
    return sum_valid_match_ids(matches, limit);
}

fn solve2(input: &str) -> u32 {
    let matches = parse_matches(input);
    return sum_powers(matches);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let limit = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
    let output = solve(input.as_str(), limit);
    println!("{output}");

    let output2 = solve2(input.as_str());
    println!("{output2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                           Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                           Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                           Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                           Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let limit = HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);
        let expected_output = 8;

        let output = solve(input, limit);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                           Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                           Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                           Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                           Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected_output = 2286;

        let output = solve2(input);

        assert_eq!(expected_output, output);
    }
}
