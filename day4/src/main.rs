use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input.txt")?;

    let mut slots: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();
    //let solution = is_eligible(slots);
    let mut total_sum = 0;
    let mut n_to_be_removed = 0;
    let solution = is_eligible_index(&mut slots);
    println!("solution is {}", solution);
    Ok(())
}

fn is_eligible(input_arr: Vec<Vec<char>>) -> i32 {
    let mut total_count = 0;
    let lenx = input_arr.len();
    let leny = input_arr[0].len();
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for r in 0..lenx {
        for c in 0..leny {
            if input_arr[r][c] != '@' {
                continue;
            }

            let mut neighbours = 0;

            for (dr, dc) in deltas {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < lenx as isize && nc >= 0 && nc < leny as isize {
                    if input_arr[nr as usize][nc as usize] == '@' {
                        neighbours += 1;
                    }
                }
            }

            if neighbours < 4 {
                total_count += 1;
            }
        }
    }
    total_count
}

fn is_eligible_index(input_arr: &mut Vec<Vec<char>>) -> usize {
    let lenx = input_arr.len();
    let leny = input_arr[0].len();

    let mut total_remove_count = 0;
    let deltas = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    loop {
        let mut indexes: Vec<(usize, usize)> = Vec::new();
        for r in 0..lenx {
            for c in 0..leny {
                if input_arr[r][c] != '@' {
                    continue;
                }

                let mut neighbours = 0;

                for (dr, dc) in deltas {
                    let nr = r as isize + dr;
                    let nc = c as isize + dc;

                    if nr >= 0 && nr < lenx as isize && nc >= 0 && nc < leny as isize {
                        if input_arr[nr as usize][nc as usize] == '@' {
                            neighbours += 1;
                        }
                    }
                }

                if neighbours < 4 {
                    indexes.push((r, c));
                }
            }
        }

        if indexes.is_empty() {
            break;
        }

        total_remove_count += indexes.len();
        for (r, c) in indexes {
            input_arr[r][c] = '.';
        }
    }

    total_remove_count
}
