use std::collections::HashMap;
use std::io;
use std::io::Read;

#[derive(Default, Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    points: Vec<(usize, usize)>,
}

impl Claim {
    // Example definition - #1293 @ 698,806: 19x29
    pub fn from_definition(def: &str) -> Result<Claim, Box<dyn std::error::Error>> {
        let stripped: String = def.chars().filter(|c| *c != ' ').collect();

        let mut res = Claim::default();
        let mut buf = String::new();

        for c in stripped.chars().enumerate() {
            let mut was_numeric = false;
            match c.1 {
                '@' => {
                    res.id = buf.parse()?;
                }
                ',' => {
                    res.x = buf.parse()?;
                }
                ':' => {
                    res.y = buf.parse()?;
                }
                'x' => {
                    res.width = buf.parse()?;
                }
                '0'..='9' => {
                    buf.push(c.1);
                    was_numeric = true
                }
                _ => {}
            }

            if c.0 == stripped.len() - 1 {
                res.height = buf.parse()?;
            }

            if !was_numeric {
                buf.clear();
            }
        }

        let mut points = Vec::with_capacity(res.width * res.height);
        for x in res.x..res.x + res.width {
            for y in res.y..res.y + res.height {
                points.push((x, y));
            }
        }
        res.points = points;

        Ok(res)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    println!("{}", part_one(&input)?);
    println!("{}", part_two2(&input)?);

    Ok(())
}

fn part_one(input: &String) -> Result<i32, Box<dyn std::error::Error>> {
    let mut occupied: HashMap<(usize, usize), i32> = HashMap::new();

    let mut total = 0;

    for line in input.lines() {
        let claim = Claim::from_definition(&line)?;
        for point in claim.points {
            let current = *occupied.get(&point).unwrap_or_else(|| &0);
            let new = current + 1;
            occupied.insert(point, new);
            if new == 2 {
                total += 1;
            }
        }
    }

    Ok(total)
}

fn part_two2(input: &String) -> Result<usize, String> {
    let mut occupied: HashMap<(usize, usize), i32> = HashMap::new();
    let mut claims = Vec::new();

    for line in input.lines() {
        let claim =
            Claim::from_definition(&line).map_err(|_| "Failed to parse claim".to_string())?;
        for point in &claim.points {
            let current = *occupied.get(&point).unwrap_or_else(|| &0);
            let new = current + 1;
            occupied.insert(point.clone(), new);
        }
        claims.push(claim);
    }

    'outer: for claim in &claims {
        for point in &claim.points {
            if *occupied.get(&point).unwrap_or_else(|| &0) > 1 {
                continue 'outer;
            }
        }
        return Ok(claim.id);
    }

    Err("No results found".to_string())
}
