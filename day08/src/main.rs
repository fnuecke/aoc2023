use std::collections::HashMap;

fn parse_direction(ch: char) -> u8 {
    match ch {
        'L' => 0,
        'R' => 1,
        _ => panic!(),
    }
}

fn parse_directions(input: &str) -> Vec<u8> {
    input.chars().map(parse_direction).collect()
}

fn encode_char(ch: char) -> u32 {
    ch as u32 - b'A' as u32
}

fn decode_char(id: u32) -> char {
    char::from_u32(b'A' as u32 + id).unwrap()
}

const RADIX: u32 = b'Z' as u32 - b'A' as u32 + 1;

fn encode_id(chars: &str) -> u32 {
    assert!(chars.len() == 3);
    chars
        .chars()
        .rev()
        .enumerate()
        .map(|(idx, ch)| encode_char(ch) * RADIX.pow(idx as u32))
        .sum()
}

#[allow(unused)]
fn decode_id(id: u32) -> String {
    let mut src = id;
    let mut str = String::new();
    for _ in 0..3 {
        str.insert(0, decode_char(src % RADIX));
        src /= RADIX;
    }
    return str;
}

fn is_start(id: u32) -> bool {
    decode_char(id % RADIX) == 'A'
}

fn is_goal(id: u32) -> bool {
    decode_char(id % RADIX) == 'Z'
}

fn parse_node(input: &str) -> (u32, [u32; 2]) {
    let id = encode_id(&input[0..3]);
    let left_start = "XXX = (".len();
    let left = encode_id(&input[left_start..(left_start + 3)]);
    let right_start = "XXX = (XXX, ".len();
    let right = encode_id(&input[right_start..(right_start + 3)]);
    (id, [left, right])
}

fn parse_input(input: &str) -> (Vec<u8>, HashMap<u32, [u32; 2]>) {
    let mut lines = input.lines().map(|it| it.trim());
    let directions = parse_directions(lines.next().unwrap().trim());
    let nodes = lines.skip(1).map(parse_node).collect();
    (directions, nodes)
}

fn solve(input: &str) -> u32 {
    let (directions, nodes) = parse_input(input);
    let mut current = encode_id("AAA");
    let goal = encode_id("ZZZ");
    let mut steps = 0;
    while current != goal {
        let direction = directions[steps % directions.len()];
        let node = nodes.get(&current).unwrap();
        current = node[direction as usize];
        steps += 1;
    }
    return steps as u32;
}

fn solve2(input: &str) -> u64 {
    let (directions, nodes) = parse_input(input);
    let starts: Vec<u32> = nodes.keys().filter(|it| is_start(**it)).copied().collect();
    let mut cycle_lengths: Vec<u64> = Vec::new();
    for idx in 0..starts.len() {
        let mut current = starts[idx];
        let mut steps = 0;
        while !is_goal(current) {
            let direction = directions[steps % directions.len()];
            let node = nodes.get(&current).unwrap();
            current = node[direction as usize];
            steps += 1;
        }
        cycle_lengths.push(steps as u64);
    }

    return lcmx::lcmx(&cycle_lengths).unwrap();
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
    fn encode_decode() {
        assert_eq!(encode_char('A'), 0);
        assert_eq!(encode_char('B'), 1);
        assert_eq!(encode_char('Z'), 25);

        assert_eq!(decode_char(0), 'A');
        assert_eq!(decode_char(1), 'B');
        assert_eq!(decode_char(25), 'Z');

        assert_eq!(encode_id("AAA"), 0);
        assert_eq!(decode_id(encode_id("AAA")), "AAA");
        assert_eq!(decode_id(encode_id("BBA")), "BBA");
        assert_eq!(decode_id(encode_id("BBZ")), "BBZ");
        assert_eq!(decode_id(encode_id("ZZZ")), "ZZZ");
    }

    #[test]
    fn sample() {
        let input = "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)";
        let expected_output = 2;

        let output = solve(input);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample_looping() {
        let input = "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)";
        let expected_output = 6;

        let output = solve(input);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample2() {
        let input = "LR

        AAA = (AAB, XXX)
        AAB = (XXX, AAZ)
        AAZ = (AAB, XXX)
        BBA = (BBB, XXX)
        BBB = (BBC, BBC)
        BBC = (BBZ, BBZ)
        BBZ = (BBB, BBB)
        XXX = (XXX, XXX)";
        let expected_output = 6;

        let output = solve2(input);

        assert_eq!(expected_output, output);
    }
}
