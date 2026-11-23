use advent_of_code_2026::io;

fn part1(_input: &str) -> i64 {
    0
}

fn part2(_input: &str) -> i64 {
    0
}

fn main() {
    let input = io::read_text("inputs/day01.txt");
    println!("Day 01, part 1: {}", part1(&input));
    println!("Day 01, part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
line1
line2
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 0);
    }
}
