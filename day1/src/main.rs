use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Instruction {
    action: char,
    value: i32,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let action = chars.next().ok_or("Empty String")?;

        let value = chars
            .as_str()
            .parse::<i32>()
            .map_err(|_| "Invalid numbers")?;

        Ok(Instruction { action, value })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input.txt")?;

    let instructions: Vec<Instruction> = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| line.trim().parse())
        .collect::<Result<Vec<Instruction>, _>>()?;

    //let solution: Vec<(i32, i32)> = solve(instructions, 50)?;
    //let zero: usize = solution.iter().filter(|(pos, wrap)| *pos == 0).count();
    //let passes: i32 = solution.iter().map(|(_pos, wrap)| wrap).sum();
    let solution = solve_fold(instructions, 50);
    println!("result zero do have : {:?}", solution);

    Ok(())
}

fn solve(instructions: Vec<Instruction>, start: i32) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    instructions
        .into_iter()
        .scan(start, |state, instr| {
            let raw_val = match instr.action {
                'L' => *state - instr.value,
                'R' => *state + instr.value,
                c => return Some(Err(format!("Unknown action: {}", c))),
            };

            let wraps = raw_val.div_euclid(100).abs();
            *state = raw_val.rem_euclid(100);

            Some(Ok((*state, wraps)))
        })
        .collect::<Result<Vec<(i32, i32)>, _>>()
        .map_err(|e| e.into())
}

fn solve_fold(instructions: Vec<Instruction>, start: i32) -> Result<i32, Box<dyn Error>> {
    let (final_pos, total_zeros) =
        instructions
            .into_iter()
            .try_fold((start, 0), |(current_pos, total_count), instr| {
                let (next_pos, zero_this_turn) = match instr.action {
                    'R' => {
                        let end_pos = current_pos + instr.value;
                        let count = end_pos.div_euclid(100) - current_pos.div_euclid(100);
                        (end_pos, count)
                    }
                    'L' => {
                        let end_pos = current_pos - instr.value;
                        let count =
                            (current_pos - 1).div_euclid(100) - (end_pos - 1).div_euclid(100);
                        (end_pos, count)
                    }
                    c => return Err(format!("Unknown action: {}", c)),
                };

                let normalized_pos = next_pos.rem_euclid(100);
                Ok((normalized_pos, total_count + zero_this_turn))
            })?;
    Ok(total_zeros)
}
