mod part1 {
    use std::{collections::HashSet, fs::read_to_string};

    pub fn create_matrix() -> Vec<Vec<char>> {
        let input = read_to_string("input.txt").unwrap();
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect()
    }

    pub fn sol() -> u32 {
        let matrix = create_matrix();
        let target = "XMAS".chars().into_iter().collect::<Vec<char>>();

        let cells = (0..matrix.len())
            .flat_map(|row| (0..matrix[0].len()).map(move |col| (row as i32, col as i32)));
        let directions = (-1..2)
            .flat_map(|row| (-1..2).map(move |col| (row as i32, col as i32)))
            .filter(|dir| dir != &(0, 0));

        // there must have been a nicer way to do this rip
        cells
            .flat_map(|(row, col)| {
                directions
                    .clone()
                    .map(|(row_jump, col_jump)| {
                        (0..(target.len() as i32))
                            .map(|index| {
                                let mrow = row + row_jump * index;
                                let mcol = col + col_jump * index;
                                if mrow < 0
                                    || mrow >= matrix.len() as i32
                                    || mcol < 0
                                    || mcol >= matrix[0].len() as i32
                                {
                                    false
                                } else {
                                    matrix[mrow as usize][mcol as usize] == target[index as usize]
                                }
                            })
                            .all(|x| x) as u32
                    })
                    .collect::<Vec<u32>>()
            })
            .sum()
    }
}

mod part2 {
    use crate::part1;

    pub fn sol() -> u32 {
        let matrix = part1::create_matrix();
        let cells = (0..matrix.len() - 2)
            .flat_map(|row| (0..matrix[0].len() - 2).map(move |col| (row as usize, col as usize)));
        cells.map(|(row, col)| {
            let mas1: String = (0..3)
                .map(|index| matrix[row + index][col + index])
                .collect();
            let mas2: String = (0..3)
                .map(|index| matrix[row + index][col + 2 - index])
                .collect();
            let mas1_rev: String = mas1.chars().rev().collect();
            let mas2_rev: String = mas2.chars().rev().collect();

            ((mas1 == "MAS" || mas1_rev == "MAS") && (mas2 == "MAS" || mas2_rev == "MAS")) as u32
        }).sum()
    }
}

fn main() {
    println!("{:?}", part1::sol());
    println!("{:?}", part2::sol());
}
