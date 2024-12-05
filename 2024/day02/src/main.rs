use std::fs::read_to_string;

fn parse_reports() -> Vec<Vec<i32>> {
    read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.split(' '))
        .map(|split_line| split_line.map(|val| val.parse().unwrap()).collect())
        .collect()
}

mod part1 {
    use crate::parse_reports;

    pub fn part1_check_report(report: Vec<i32>) -> bool {
        report
            .windows(2)
            .all(|slice| -3 <= slice[1] - slice[0] && slice[1] - slice[0] <= -1)
            || report
                .windows(2)
                .all(|slice| 1 <= slice[1] - slice[0] && slice[1] - slice[0] <= 3)
    }

    pub fn part1_solution() -> u32 {
        parse_reports()
            .into_iter()
            .map(part1_check_report)
            .map(|value| value as u32)
            .collect::<Vec<u32>>()
            .into_iter()
            .sum::<u32>()
    }
}

mod part2 {
    use crate::{
        parse_reports,
        part1::{self, part1_check_report},
    };

    fn part2_check_report(report: Vec<i32>) -> bool {
        if part1::part1_check_report(report.clone()) {
            return true;
        }

        (0..report.len())
            .map(|index| part1_check_report([&report[..index], &report[(index + 1)..]].concat())).any(|x| x)

    }

    pub fn part2_solution() -> u32 {
        parse_reports()
            .into_iter()
            .map(part2_check_report)
            .map(|value| value as u32)
            .collect::<Vec<u32>>()
            .into_iter()
            .sum::<u32>()
    }
}

fn main() {
    println!("{:?}", part1::part1_solution());
    println!("{:?}", part2::part2_solution());
}
