fn fully_contains(a: (usize, usize), b: (usize, usize)) -> usize {
    if (a.0 <= b.0 && a.1 >= b.1) || (b.0 <= a.0 && b.1 >= a.1) {
        1
    } else {
        0
    }
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            let ((a, b), (c, d)) = (a.split_once("-").unwrap(), b.split_once("-").unwrap());
            (
                a.parse().unwrap(),
                b.parse().unwrap(),
                c.parse().unwrap(),
                d.parse().unwrap(),
            )
        })
        .map(|(a, b, c, d)| fully_contains((a, b), (c, d)))
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod test {
    use crate::solve;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn example() {
        assert_eq!(solve(INPUT), 2);
    }
}
