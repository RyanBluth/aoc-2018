use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<(), String> {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    println!("{}", part_one(&input)?);
    println!("{}", part_two(&input)?);

    Ok(())
}

fn part_one(input: &String) -> Result<i32, String> {
    Ok(input
        .split("\n")
        .map(|val| val.parse::<i32>())
        .sum::<Result<i32, _>>()
        .map_err(|_| "Failed to parse value from input")?)
}

fn part_two(input: &String) -> Result<i32, String> {
    let mut found_frequencies = HashSet::new();

    let frequencies: Vec<Result<i32, _>> =
        input.split("\n").map(|val| val.parse::<i32>()).collect();

    let mut current = 0;

    loop {
        for frequency in &frequencies {
            current += frequency
                .as_ref()
                .map_err(|e| format!("Failed to parse value from input: {}", e))?;
            if found_frequencies.contains(&current) {
                return Ok(current);
            } else {
                found_frequencies.insert(current);
            }
        }
    }
}
