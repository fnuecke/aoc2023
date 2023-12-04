fn parse_numbers(input: &str) -> Vec<u32> {
    return input
        .split_whitespace()
        .map(|it| it.parse().unwrap())
        .collect();
}

fn parse_line(input: &str) -> (Vec<u32>, Vec<u32>) {
    let parts: Vec<&str> = input.trim().split(":").collect();
    let numbers: Vec<&str> = parts[1].split("|").collect();
    let winning_numbers: Vec<u32> = parse_numbers(numbers[0].trim());
    let have_numbers: Vec<u32> = parse_numbers(numbers[1].trim());
    return (winning_numbers, have_numbers);
}

fn count_wins(winning_numbers: &Vec<u32>, have_numbers: &Vec<u32>) -> u32 {
    let mut wins = 0;
    for number in have_numbers {
        for winner in winning_numbers {
            if *number == *winner {
                wins += 1;
                break;
            }
        }
    }
    return wins;
}

fn solve(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let (winning_numbers, have_numbers) = parse_line(line);

        let wins = count_wins(&winning_numbers, &have_numbers);

        if wins > 0 {
            sum += (2u32).pow(wins - 1);
        }
    }
    return sum;
}

fn solve2(input: &str) -> u32 {
    let mut sum = 0;
    let mut open_intervals: Vec<u32> = Vec::new(); // current open intervals defined by remaining items in interval
    for line in input.lines() {
        let (winning_numbers, have_numbers) = parse_line(line);

        sum += 1; // this card

        let extras = u32::try_from(open_intervals.len()).unwrap();
        open_intervals.retain_mut(|it| {
            *it -= 1;
            *it > 0
        });
        sum += extras;

        let wins = count_wins(&winning_numbers, &have_numbers);

        if wins > 0 {
            open_intervals.push(wins);
            for _ in 0..extras {
                // copies win as well
                open_intervals.push(wins);
            }
        }
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

    #[test]
    fn sample() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected_output = 13;

        let output = solve(input);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
                     Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
                     Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
                     Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
                     Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
                     Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let expected_output = 30;

        let output = solve2(input);

        assert_eq!(expected_output, output);
    }
}
