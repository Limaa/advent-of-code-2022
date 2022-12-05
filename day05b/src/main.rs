fn process_stacks(input: &str) -> Vec<Vec<u8>> {
    let mut input = input.lines().rev();
    let num_stacks = input.next().unwrap().split_ascii_whitespace().count();

    let mut stacks = vec![Vec::new(); num_stacks];
    for l in input {
        let l = l.as_bytes();
        for x in 0..num_stacks {
            let value = l[x + 1 + x * 3];
            if value != b' ' {
                stacks[x].push(value);
            }
        }
    }
    stacks
}

fn process_procedures(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .map(|l| {
            let words = l.split_ascii_whitespace().collect::<Vec<_>>();
            (
                words[1].parse().unwrap(),
                words[3].parse().unwrap(),
                words[5].parse().unwrap(),
            )
        })
        .collect()
}

fn solve(input: &str) -> String {
    let (stacks, procedures) = input.split_once("\n\n").unwrap();
    let mut stacks = process_stacks(stacks);
    let procedures = process_procedures(procedures);

    for p in procedures {
        let at = stacks[p.1 - 1].len() - p.0;
        let mut values = stacks[p.1 - 1].split_off(at);
        stacks[p.2 - 1].append(&mut values);
    }

    let result = stacks
        .iter()
        .map(|x| *(x.last().unwrap()))
        .collect::<Vec<_>>();
    String::from_utf8(result).unwrap()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod test {
    use crate::solve;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn example() {
        assert_eq!(solve(INPUT), "MCD");
    }
}
