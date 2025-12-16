use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Interval {
    start: u64,
    end: u64,
}

impl FromStr for Interval {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .trim()
            .split_once('-')
            .ok_or_else(|| format!("Invalid format: {}", s))?;

        let start = start
            .parse::<u64>()
            .map_err(|_| "Invalid start number".to_string())?;
        let end = end
            .parse::<u64>()
            .map_err(|_| "Invalid end number".to_string())?;

        Ok(Interval { start, end })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string("input.txt")?;

    let intervals: Vec<Interval> = content
        .split(',')
        .map(|i| i.parse())
        .collect::<Result<Vec<Interval>, _>>()?;
    let solution = intervals
        .into_iter()
        .map(|i| solve(i))
        .sum::<Result<u64, _>>()?;

    println!("result zero do have : {:?}", solution);

    Ok(())
}

fn solve(intervals: Interval) -> Result<u64, Box<dyn Error>> {
    let numbers = (intervals.start..(intervals.end + 1_u64))
        //.filter(|&n| is_symmetric_numbers(n))
        .filter(|&n| is_pattern_repeating(n))
        .sum();
    Ok(numbers)
}

fn is_symmetric_numbers(n: u64) -> bool {
    if n == 0 {
        return false;
    }
    let digits = n.ilog10() + 1;

    if digits % 2 != 0 {
        return false;
    }

    let divisor = 10_u64.pow(digits / 2);
    n / divisor == n % divisor
}

fn is_pattern_repeating(n: u64) -> bool {
    let s = n.to_string();
    let ss = format!("{}{}", s, s);
    let search_area = &ss[1..ss.len() - 1];
    search_area.contains(&s)
}
