use std::collections::HashMap;
use std::io::Read;
use std::{cmp, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));

    Ok(())
}

fn part_one(input: &String) -> usize {
    react(input).len()
}

fn part_two(input: &String) -> usize {
    let mut min_len = std::usize::MAX;

    for ignore_char in ('a' as u32..='z' as u32).filter_map(std::char::from_u32) {
        min_len = cmp::min(
            react(
                &input
                    .chars()
                    .filter(|c| *c != ignore_char && *c != ignore_char.to_ascii_uppercase())
                    .collect(),
            )
            .len(),
            min_len,
        );
    }
    min_len
}

fn react(input: &String) -> String {
    let mut skip_next = false;
    let mut res = String::new();

    let mut iter = input.chars().peekable();

    while let Some(c) = iter.next() {
        let peek_char = iter.peek();
        if let Some(next_char) = peek_char {
            if !skip_next {
                if next_char.eq_ignore_ascii_case(&c) && c != *next_char {
                    skip_next = true;
                    continue;
                } else {
                    res.push(c);
                }
            }
            skip_next = false;
        } else if !skip_next {
            res.push(c);
        }
    }

    if res.len() == input.len() {
        return res;
    } else {
        return react(&res);
    }
}
