fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let output = input
        .lines()
        .map(|line| {
            let mut it = line.chars().filter_map(|char| char.to_digit(10));
            let first = it.next().expect("Should be a number, it's not!");
            match it.last() {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("Should be a number!")
        })
        .sum::<u32>();
    return output.to_string();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
        assert_eq!(part1(input), "142");
    }
}
