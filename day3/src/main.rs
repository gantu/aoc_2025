use std::error::Error;
use std::fs;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input.txt")?;

    let slots: Vec<Vec<i32>> = content
        .lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as i32).collect())
        .collect();

    //   let result: i32 = slots
    //       .iter()
    //       .map(|slot| {
    //           let (a1, a2) = solve_slot(slot);
    //           (a1 * 10) + a2
    //       })
    //       .sum();

    let result: u64 = slots
        .iter()
        .map(|slot| {
            let t = solve_largest_subsequence(slot, slot.len() - 12);
            vec_to_u64_string(&t)
        })
        .sum();
    println!("results is {}", result);
    Ok(())
}

fn two_biggest_fold(input: &[i32]) -> (i32, i32) {
    let start_state = ((-1, 0), (-1, 0));

    let (champion, runner_up) =
        input
            .iter()
            .enumerate()
            .fold(start_state, |(best, second), (idx, &val)| {
                if val >= best.0 {
                    ((val, idx), best)
                } else if val > second.0 {
                    (best, (val, idx))
                } else {
                    (best, second)
                }
            });

    if champion.1 < runner_up.1 {
        (champion.0, runner_up.0)
    } else {
        (runner_up.0, champion.0)
    }
}
fn solve_slot(input: &[i32]) -> (i32, i32) {
    let mut max_suffix = -1; // The biggest number to the RIGHT of current position
    let mut best_score = -1;
    let mut best_pair = (0, 0);

    // Iterate Backwards: 3, 2, 1, 0...
    for &val in input.iter().rev() {
        // 1. Calculate score with the best number found to the right so far
        if max_suffix != -1 {
            let score = val * 10 + max_suffix;
            if score >= best_score {
                best_score = score;
                best_pair = (val, max_suffix);
            }
        }

        // 2. Update the "biggest number to the right" for the next step
        if val > max_suffix {
            max_suffix = val;
        }
    }

    best_pair
}

fn solve_largest_subsequence(input: &[i32], to_remove: usize) -> Vec<i32> {
    let mut stack: Vec<i32> = Vec::new();
    let mut removals_left = to_remove;

    for &digit in input {
        while removals_left > 0 && !stack.is_empty() && digit > *stack.last().unwrap() {
            stack.pop();
            removals_left -= 1;
        }
        stack.push(digit);
    }

    stack.truncate(stack.len() - removals_left);

    stack
}

fn vec_to_u64_string(digits: &[i32]) -> u64 {
    // 1. Convert each digit to char/string and join them
    let s: String = digits.iter().map(|d| d.to_string()).collect::<String>();

    // 2. Parse
    s.parse::<u64>().expect("Number too large for u64")
}
