use chrono::offset::TimeZone;
use chrono::{DateTime, Timelike, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};
use std::io;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
struct StringError(String);

impl Display for StringError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for StringError {}

#[derive(Debug, PartialEq, Clone)]
enum Event {
    WakeUp,
    FallAsleep,
    NewGuard(usize),
}

#[derive(Debug, Clone)]
struct LogEntry {
    date: DateTime<Utc>,
    event: Event,
}

impl FromStr for LogEntry {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\[(?P<date>.*)\]\s*?(?P<event>wakes up|falls asleep|Guard \#(?P<id>[0-9].*) begins shift)").unwrap();
        }

        match RE.captures(s) {
            Some(cap) => {
                let date = cap
                    .name("date")
                    .ok_or(Box::new(StringError("Failed to extract date".to_string())))
                    .map(|date_match| {
                        Utc.datetime_from_str(date_match.as_str(), "%Y-%m-%d %H:%M")
                            .map_err(|e| {
                                format!(
                                    "Failed to parse date {} : {}",
                                    date_match.as_str(),
                                    e.to_string()
                                )
                            })
                    })??;

                let event = cap
                    .name("event")
                    .ok_or(StringError("Failed to extract event".to_string()))
                    .map(|event| {
                        let event_str = event.as_str();
                        if event_str == "wakes up" {
                            return Ok(Event::WakeUp);
                        } else if event_str == "falls asleep" {
                            return Ok(Event::FallAsleep);
                        } else {
                            return cap
                                .name("id")
                                .ok_or(StringError("Failed to extract id".to_string()))
                                .and_then(|id| {
                                    id.as_str()
                                        .parse::<usize>()
                                        .and_then(|id| Ok(Event::NewGuard(id)))
                                        .map_err(|e| {
                                            StringError(format!(
                                                "Failed to parse ID {} : {}",
                                                id.as_str(),
                                                e.to_string()
                                            ))
                                        })
                                });
                        }
                    })??;

                return Ok(LogEntry { date, event });
            }
            None => return Err(Box::new(StringError("No Captures".to_string()))),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| format!("Failed to read input: {}", e))?;

    let mut entries = Vec::new();

    for line in input.lines() {
        entries.push(LogEntry::from_str(line)?);
    }

    entries.sort_by(|a, b| a.date.cmp(&b.date));

    println!("{}", part_one(&entries)?);
    println!("{}", part_two(&entries)?);

    Ok(())
}

fn part_one(entries: &Vec<LogEntry>) -> Result<usize, Box<dyn std::error::Error>> {
    let minutes = calculate_minutes_asleep(entries);

    let mut total_times: Vec<(usize, usize)> = minutes
        .iter()
        .map(|entry| (*entry.0, entry.1.values().map(|x| *x).sum::<usize>()))
        .collect();

    total_times.sort_by(|a, b| b.1.cmp(&a.1));

    let mut max_min = (0, 0);

    let id = total_times[0].0;

    for min in minutes.get(&id).unwrap() {
        if *min.1 > max_min.1 {
            max_min = (*min.0, *min.1);
        }
    }

    Ok(id * max_min.0 as usize)
}

fn part_two(entries: &Vec<LogEntry>) -> Result<usize, Box<dyn std::error::Error>> {
    let minutes = calculate_minutes_asleep(entries);

    // The minute with the most sleep time
    let mut max_minute = 0;

    // The total time asleep for a max_minute
    let mut max_time_asleep = 0;

    // The guard with the most sleep time on max minute
    let mut max_guard = 0;

    for entry in &minutes {
        for minute in entry.1 {
            if *minute.1 > max_time_asleep {
                max_time_asleep = *minute.1;
                max_minute = *minute.0;
                max_guard = *entry.0;
            }
        }
    }

    Ok(max_guard * max_minute as usize)
}

fn calculate_minutes_asleep(entries: &Vec<LogEntry>) -> HashMap<usize, HashMap<u32, usize>> {
    let mut last_entry: Option<LogEntry> = None;
    let mut cur_guard: Option<usize> = None;

    let mut minutes: HashMap<usize, HashMap<u32, usize>> = HashMap::new();

    for entry in entries {
        match entry.event {
            Event::WakeUp => {
                if let Some(last_entry) = &last_entry {
                    if let Some(some_guard) = &cur_guard {
                        for minute in last_entry.date.minute()..entry.date.minute() {
                            if minutes.get(&some_guard).is_none() {
                                minutes.insert(*some_guard, HashMap::new());
                            }
                            let guard_mins = minutes.get_mut(&some_guard).unwrap();
                            let current_count = *guard_mins.get(&minute).unwrap_or(&0);
                            guard_mins.insert(minute, current_count + 1);
                        }
                    }
                }
            }
            Event::NewGuard(id) => {
                cur_guard = Some(id);
            }
            Event::FallAsleep => {
                last_entry = Some(entry.clone());
            }
        }
    }

    minutes
}
