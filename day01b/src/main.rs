fn solve(input: &str) -> usize {
    let mut elves: Vec<usize> = input
        .split("\n\n")
        .map(|calories| {
            calories
                .lines()
                .map(|calorie| calorie.parse::<usize>().unwrap())
                .sum()
        })
        .collect();
    elves.sort_unstable();
    elves.iter().rev().take(3).sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod test {
    use crate::solve;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn example() {
        assert_eq!(solve(INPUT), 45000);
    }
}
