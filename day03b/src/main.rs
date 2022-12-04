fn priority(item: u8) -> u8 {
    if item > 'Z' as u8 {
        (item - b'a') + 1
    } else {
        (item - b'A') + 27
    }
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|elves| {
            elves[0]
                .iter()
                .find(|x| elves[1].contains(x) && elves[2].contains(x))
                .unwrap()
        })
        .map(|x| priority(*x) as usize)
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod test {
    use crate::solve;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn example() {
        assert_eq!(solve(INPUT), 70);
    }
}
