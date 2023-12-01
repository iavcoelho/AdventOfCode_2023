fn main() {
    let input = include_str!("input.txt");
    let output = part1(input);
    dbg!(output);
}

fn process(_input: &str) -> i32 {
    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let input = "";
        assert_eq!(process(input), 4);
    }
}
