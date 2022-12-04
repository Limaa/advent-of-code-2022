enum Shape {
    Rock,
    Paper,
    Scissor,
}

fn shape_from_str(shape: &str) -> Shape {
    match shape {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissor,
        "X" => Shape::Rock,
        "Y" => Shape::Paper,
        "Z" => Shape::Scissor,
        _ => unreachable!(),
    }
}

enum Outcome {
    Lost,
    Draw,
    Won,
}

fn outcome(oponent: Shape, me: Shape) -> Outcome {
    match oponent {
        Shape::Rock => match me {
            Shape::Rock => Outcome::Draw,
            Shape::Paper => Outcome::Won,
            Shape::Scissor => Outcome::Lost,
        },
        Shape::Paper => match me {
            Shape::Rock => Outcome::Lost,
            Shape::Paper => Outcome::Draw,
            Shape::Scissor => Outcome::Won,
        },
        Shape::Scissor => match me {
            Shape::Rock => Outcome::Won,
            Shape::Paper => Outcome::Lost,
            Shape::Scissor => Outcome::Draw,
        },
    }
}

fn score(oponent: Shape, me: Shape) -> usize {
    let shape_selected = match me {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    };

    let round_outcome = match outcome(oponent, me) {
        Outcome::Lost => 0,
        Outcome::Draw => 3,
        Outcome::Won => 6,
    };

    shape_selected + round_outcome
}

fn solve(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(oponent, me)| (shape_from_str(oponent), shape_from_str(me)))
        .map(|(oponent, me)| score(oponent, me))
        .sum()
}

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve(input));
}

#[cfg(test)]
mod test {
    use crate::solve;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn example() {
        assert_eq!(solve(INPUT), 15);
    }
}
