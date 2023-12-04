use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let final_val: usize = input
        .lines()
        .map(|line| line.split([':', '|']))
        .map(|mut tokens| {
            let _ = tokens.next();
            let a = tokens
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .collect::<HashSet<&str>>();
            let b = tokens
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .collect::<HashSet<&str>>();
            let inter = a.intersection(&b).count();
            if inter == 0 {
                0
            } else {
                1 << inter - 1
            }
        })
        .sum();
    final_val.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(process(input), "13");
    }
}
