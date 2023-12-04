#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

struct Number {
    value: u32,
    positions: Vec<Point>,
}

struct Symbol {
    value: char,
    position: Point,
}

fn is_adjacent(p1: Point, p2: Point) -> bool {
    let dx = (p1.x).abs_diff(p2.x);
    let dy = (p1.y).abs_diff(p2.y);
    return dx <= 1 && dy <= 1;
}

fn parse(input: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    let mut position = Point { x: 0, y: 0 };
    for line in input.lines() {
        let mut number: Number = Number {
            value: 0,
            positions: Vec::new(),
        };
        for ch in line.trim().chars() {
            if ch.is_numeric() {
                let val: u32 = format!("{ch}").parse().unwrap();
                number.value = number.value * 10 + val;
                number.positions.push(position);
            } else {
                if !number.positions.is_empty() {
                    numbers.push(number);
                    number = Number {
                        value: 0,
                        positions: Vec::new(),
                    };
                }
                if ch != '.' {
                    symbols.push(Symbol {
                        value: ch,
                        position: position.clone(),
                    });
                }
            }
            position.x += 1;
        }
        if !number.positions.is_empty() {
            numbers.push(number);
        }
        position.x = 0;
        position.y += 1;
    }
    return (numbers, symbols);
}

fn solve(input: &str) -> u32 {
    let (numbers, symbols) = parse(input);

    let mut sum = 0;
    'outer: for number in numbers {
        for position in number.positions {
            for symbol in symbols.iter() {
                if is_adjacent(position, symbol.position) {
                    sum += number.value;
                    continue 'outer;
                }
            }
        }
    }

    return sum;
}

fn solve2(input: &str) -> u32 {
    let (numbers, symbols) = parse(input);

    let mut sum = 0;
    for symbol in symbols {
        if symbol.value != '*' {
            continue;
        }

        let mut gear_ratio = 1;
        let mut count = 0;
        for number in numbers.iter() {
            for position in number.positions.iter() {
                if is_adjacent(*position, symbol.position) {
                    gear_ratio *= number.value;
                    count += 1;
                    break;
                }
            }
        }

        if count == 2 {
            sum += gear_ratio;
        }
    }

    return sum;
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
    fn sample() {
        let input = "467..114..
                     ...*......
                     ..35..633.
                     ......#...
                     617*......
                     .....+.58.
                     ..592.....
                     ......755.
                     ...$.*....
                     .664.598..";
        let expected_output = 4361;

        let output = solve(input);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample2() {
        let input = "467..114..
                     ...*......
                     ..35..633.
                     ......#...
                     617*......
                     .....+.58.
                     ..592.....
                     ......755.
                     ...$.*....
                     .664.598..";
        let expected_output = 467835;

        let output = solve2(input);

        assert_eq!(expected_output, output);
    }
}
