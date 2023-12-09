use itertools::{self, Itertools};
use std::ops::Range;

fn solve(input: &str) -> u64 {
    let seeds_string = input.lines().next().unwrap().trim();
    assert!(seeds_string.starts_with("seeds: "));
    let mut source_set: Vec<u64> = seeds_string["seeds: ".len()..]
        .split(" ")
        .map(|it| it.parse().unwrap())
        .collect();
    let mut destination_set: Vec<u64> = Vec::new();

    for line in input.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        if line.contains(":") {
            source_set.append(&mut destination_set);
        } else {
            let mapping: Vec<u64> = line
                .trim()
                .split(" ")
                .map(|it| it.parse().unwrap())
                .collect();
            let dst_base = mapping[0];
            let src_base = mapping[1];
            let map_count = mapping[2];
            let map = src_base..(src_base + map_count);

            source_set.retain(|src| {
                if map.contains(src) {
                    destination_set.push(dst_base + src - src_base);
                    false
                } else {
                    true
                }
            });
        }
    }

    source_set.append(&mut destination_set);

    return *source_set.iter().min().unwrap();
}

fn solve2(input: &str) -> u64 {
    let seeds_string = input.lines().next().unwrap().trim();
    assert!(seeds_string.starts_with("seeds: "));
    let mut source_set: Vec<Range<u64>> = seeds_string["seeds: ".len()..]
        .split(" ")
        .map(|it| it.parse().unwrap())
        .tuples()
        .map(|(offset, count)| offset..(offset + count))
        .collect();
    let mut destination_set: Vec<Range<u64>> = Vec::new();

    for line in input.lines().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        if line.contains(":") {
            source_set.append(&mut destination_set);
        } else {
            let mapping: Vec<u64> = line
                .trim()
                .split(" ")
                .map(|it| it.parse().unwrap())
                .collect();
            let dst_base = mapping[0];
            let src_base = mapping[1];
            let map_count = mapping[2];
            let map = src_base..(src_base + map_count);

            for i in (0..source_set.len()).rev() {
                let src = source_set[i].clone();
                let intersect_start = src.start.max(map.start);
                let intersect_end = src.end.min(map.end);
                if intersect_end <= intersect_start {
                    continue;
                }

                source_set.remove(i);

                let left = src.start..intersect_start;
                let intersect = intersect_start..intersect_end;
                let right = intersect_end..src.end;

                assert!(!intersect.is_empty());
                let mapped_start = dst_base + intersect_start - src_base;
                let mapped_count = intersect.end - intersect.start;
                destination_set.push(mapped_start..(mapped_start + mapped_count));

                if !left.is_empty() {
                    source_set.push(left);
                }
                if !right.is_empty() {
                    source_set.push(right);
                }
            }
        }
    }

    source_set.append(&mut destination_set);

    return source_set.iter().map(|r| r.start).min().unwrap();
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

    const TEST_INPUT: &str = "seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48
    
    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15
    
    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4
    
    water-to-light map:
    88 18 7
    18 25 70
    
    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13
    
    temperature-to-humidity map:
    0 69 1
    1 0 69
    
    humidity-to-location map:
    60 56 37
    56 93 4
    ";

    #[test]
    fn sample() {
        let expected_output = 35;

        let output = solve(TEST_INPUT);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample2() {
        let expected_output = 46;

        let output = solve2(TEST_INPUT);

        assert_eq!(expected_output, output);
    }
}
