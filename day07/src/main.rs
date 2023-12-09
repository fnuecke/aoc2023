const CARDS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn card_value(card: char) -> u32 {
    CARDS.iter().position(|it| *it == card).unwrap() as u32
}

const CARDS2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn card_value2(card: char) -> u32 {
    CARDS2.iter().position(|it| *it == card).unwrap() as u32
}

#[derive(PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, PartialOrd)]
struct Hand {
    class: HandType,
    cards: Vec<u32>,
}

struct CardBag {
    value: u32,
    count: u32,
}

fn classify_hand(cards: &Vec<u32>) -> HandType {
    let mut bags: Vec<CardBag> = Vec::new();
    for &value in cards {
        match bags.iter_mut().find(|bag| bag.value == value) {
            Some(bag) => bag.count += 1,
            _ => bags.push(CardBag { value, count: 1 }),
        }
    }
    bags.sort_by_key(|bag| bag.count);
    match bags.len() {
        5 => HandType::HighCard,
        4 => HandType::OnePair,
        3 if bags[2].count == 3 => HandType::ThreeOfAKind,
        3 => HandType::TwoPair,
        2 if bags[1].count == 4 => HandType::FourOfAKind,
        2 => HandType::FullHouse,
        1 => HandType::FiveOfAKind,
        _ => panic!(),
    }
}

fn classify_hand2(cards: &Vec<u32>) -> HandType {
    let mut bags: Vec<CardBag> = Vec::new();
    for &value in cards {
        match bags.iter_mut().find(|bag| bag.value == value) {
            Some(bag) => bag.count += 1,
            _ => bags.push(CardBag { value, count: 1 }),
        }
    }
    let mut jokers: u32 = 0;
    bags.retain_mut(|it| {
        if it.value == card_value2('J') {
            jokers = it.count;
            false
        } else {
            true
        }
    });
    bags.sort_by_key(|bag| bag.count);
    match bags.len() {
        5 => HandType::HighCard,                                    // 1,1,1,1,1+0
        4 => HandType::OnePair,                                     // 2,1,1,1+0 or 1,1,1,1+1
        3 if bags[2].count + jokers == 3 => HandType::ThreeOfAKind, // 3,1,1+0 or 2,1,1+1 or 1,1,1+2
        3 => HandType::TwoPair,                                     // 2,2,1+0
        2 if bags[1].count + jokers == 4 => HandType::FourOfAKind,  // 4,1+0 or 3,1+1 or 2,1+2
        2 => HandType::FullHouse,                                   // 3,2+0 or 2,2+1
        1 | 0 => HandType::FiveOfAKind,
        _ => panic!(),
    }
}

fn parse_cards(input: &str) -> Vec<u32> {
    input.chars().map(card_value).collect()
}

fn parse_cards2(input: &str) -> Vec<u32> {
    input.chars().map(card_value2).collect()
}

fn parse_hand(input: &str) -> Hand {
    let cards = parse_cards(input);
    let class = classify_hand(&cards);
    return Hand { cards, class };
}

fn parse_hand2(input: &str) -> Hand {
    let cards = parse_cards2(input);
    let class = classify_hand2(&cards);
    return Hand { cards, class };
}

fn parse_hands(input: &str, parser: fn(&str) -> Hand) -> Vec<(Hand, u32)> {
    let mut hands: Vec<(Hand, u32)> = Vec::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let hand = parser(parts.next().unwrap());
        let bid: u32 = parts.next().unwrap().parse().unwrap();
        hands.push((hand, bid));
    }

    hands.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    return hands;
}

fn rank_hands(hands: &Vec<(Hand, u32)>) -> u32 {
    let mut sum = 0;
    for (index, (_, bid)) in hands.iter().enumerate() {
        let rank = index as u32 + 1;
        sum += *bid * rank;
    }

    return sum;
}

fn solve(input: &str) -> u32 {
    let hands = parse_hands(input, parse_hand);
    return rank_hands(&hands);
}

fn solve2(input: &str) -> u32 {
    let hands = parse_hands(input, parse_hand2);
    return rank_hands(&hands);
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

    const TEST_INPUT: &str = "32T3K 765
                              T55J5 684
                              KK677 28
                              KTJJT 220
                              QQQJA 483";

    #[test]
    fn classification() {
        assert_eq!(classify_hand(&parse_cards("33332")), HandType::FourOfAKind);
        assert_eq!(classify_hand(&parse_cards("2AAAA")), HandType::FourOfAKind);
        assert_eq!(classify_hand(&parse_cards("77888")), HandType::FullHouse);
        assert_eq!(classify_hand(&parse_cards("77788")), HandType::FullHouse);
        assert_eq!(classify_hand2(&parse_cards2("QJJQ2")), HandType::FourOfAKind);
        assert_eq!(classify_hand2(&parse_cards2("JKKK2")), HandType::FourOfAKind);
    }

    #[test]
    fn ordering() {
        assert!(parse_hand("33332") > parse_hand("2AAAA"));
        assert!(parse_hand("77888") > parse_hand("77788"));
        assert!(parse_hand("QQQQ2") > parse_hand("JKKK2"));
    }

    #[test]
    fn sample() {
        let expected_output = 6440;

        let output = solve(TEST_INPUT);

        assert_eq!(expected_output, output);
    }

    #[test]
    fn sample2() {
        let expected_output = 5905;

        let output = solve2(TEST_INPUT);

        assert_eq!(expected_output, output);
    }
}
