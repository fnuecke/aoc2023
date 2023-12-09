use std::ops::Range;

struct Race {
    duration: u64,
    distance: u64,
}

fn find_winning_interval(race: &Race) -> Option<Range<u64>> {
    // t = t_charge + t_move
    // v = t_charge
    // d = v * t_move
    // 0 = (t - t_move) * t_move - d
    // 0 = -t_move*t_move + t*t_move - d
    // x1/2 = (-b +/- sqrt(b*b - 4ac)) / 2a
    let a: f64 = -1.0;
    let b = race.duration as f64;
    let c = -(race.distance as f64);

    let d = b * b - 4.0 * a * c;
    if d < 0.0 {
        return None;
    }
    let sqrtd = d.sqrt();
    let x1 = (b - sqrtd) / 2.0;
    let x2 = (b + sqrtd) / 2.0;

    // We want to be better than the current best, so in case we hit
    // it exactly, step one up / down.
    let w1 = if x1.fract() == 0.0 {
        x1 as u64 + 1
    } else {
        x1.ceil() as u64
    };
    let w2 = if x2.fract() == 0.0 {
        x2 as u64 - 1
    } else {
        x2.floor() as u64
    };
    return Some(w1..(w2 + 1));
}

fn parse_input(input: &str) -> Vec<Race> {
    let mut lines = input.lines();
    let times: Vec<u64> = lines.next().unwrap().trim()["Time:".len()..]
        .trim()
        .split_whitespace()
        .map(|it| it.parse().unwrap())
        .collect();
    let distances: Vec<u64> = lines.next().unwrap().trim()["Distance:".len()..]
        .trim()
        .split_whitespace()
        .map(|it| it.parse().unwrap())
        .collect();
    return times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            duration: *time,
            distance: *distance,
        })
        .collect();
}

fn parse_input2(input: &str) -> Race {
    let mut lines = input.lines();
    let duration: u64 = lines.next().unwrap().trim()["Time:".len()..]
        .replace(char::is_whitespace, "")
        .parse()
        .unwrap();
    let distance: u64 = lines.next().unwrap().trim()["Distance:".len()..]
        .replace(char::is_whitespace, "")
        .parse()
        .unwrap();
    return Race { duration, distance };
}

fn solve(input: &str) -> u64 {
    let races = parse_input(input);
    let winning_intervals: Vec<Range<u64>> =
        races.iter().filter_map(find_winning_interval).collect();
    return winning_intervals
        .iter()
        .map(|it| it.end - it.start)
        .reduce(|a, b| a * b)
        .unwrap();
}

fn solve2(input: &str) -> u64 {
    let race = parse_input2(input);
    let winning_interval = find_winning_interval(&race).unwrap();
    return winning_interval.end - winning_interval.start;
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

    const TEST_INPUT: &str = "Time:      7  15   30
                              Distance:  9  40  200";

    #[test]
    fn sample() {
        let expected_output = 288;

        let output = solve(TEST_INPUT);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample2() {
        let expected_output = 71503;

        let output = solve2(TEST_INPUT);

        assert_eq!(expected_output, output);
    }
}
