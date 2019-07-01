use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    println!("{}", part_one(&input)?);
    println!("{}", part_two(&input)?);

    Ok(())
}

fn part_one(input: &String) -> Result<i32, String> {
    let mut two_letter_words = 0;
    let mut three_letter_words = 0;

    input.lines().for_each(|line| {
        let mut count_map = HashMap::new();
        line.chars().for_each(|c| {
            if count_map.contains_key(&c) {
                count_map.insert(c, count_map[&c] + 1);
            } else {
                count_map.insert(c, 1);
            }
        });
        let mut matched_two = false;
        let mut matched_three = false;
        for entry in count_map {
            if entry.1 == 2 && !matched_two {
                two_letter_words += 1;
                matched_two = true;
            } else if entry.1 == 3 && !matched_three {
                three_letter_words += 1;
                matched_three = true;
            }
        }
    });
    Ok(two_letter_words * three_letter_words)
}

fn part_two(input: &String) -> Result<String, String> {
    let lines: Vec<&str> = input.lines().collect();

    for line1 in &lines {
        for line2 in &lines {
            let mut diffs = Vec::new();
            line1
                .chars()
                .zip(line2.chars())
                .enumerate()
                .for_each(|pair| {
                    let idx = pair.0;
                    let c1 = (pair.1).0;
                    let c2 = (pair.1).1;
                    if c1 != c2 {
                        diffs.push(idx);
                    }
                });
            if diffs.len() == 1 {
                return Ok(line1
                    .chars()
                    .enumerate()
                    .filter_map(|x| {
                        if x.0 != diffs[0] {
                            return Some(x.1);
                        } else {
                            return None;
                        }
                    })
                    .collect());
            }
        }
    }
    Err("No match found".to_string())
}
