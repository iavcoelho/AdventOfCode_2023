use std::collections::BTreeMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    let mut game_power: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut map = BTreeMap::from([("red", 0), ("green", 0), ("blue", 0)]);
        let data = line.split(":").collect::<Vec<_>>();
        let sub_games: Vec<_> = data[1]
            .split(";")
            .flat_map(|game| game.split(&[',', ' ']).filter(|r| r.ne(&"")))
            .collect();
        for i in (0..sub_games.len()).step_by(2) {
            let size = sub_games.get(i).unwrap().parse::<i32>().unwrap();
            let color = sub_games.get(i + 1).unwrap();
            map.entry(color)
                .and_modify(|value| *value = (*value).max(size));
        }
        game_power.push(map.values().product());
    }
    game_power.iter().sum::<i32>().to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green;";

        assert_eq!(process(input), "2286");
    }
}
