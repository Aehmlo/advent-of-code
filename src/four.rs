//! Day four (Repose Record)

use std::collections::HashMap;

/// Identifies a guard. The Elves seem to give them boring numeric identifiers.
pub type GuardId = u16;

/// Represents an associated action in the logbook entry.
#[derive(Eq, PartialEq)]
pub enum Action {
    /// Marks the start of a shift.
    Start(GuardId),
    /// Marks the beginning of a nap.
    Sleep,
    /// Marks the end of a nap.
    Wake,
}

/// Finds the sleepiest guard and when they're most frequently asleep.
pub fn find_sleepy<S: ::std::hash::BuildHasher>(
    times: &HashMap<GuardId, Vec<u8>, S>,
) -> (GuardId, u8) {
    let mut sleepiest = 0;
    let mut guard = 0;
    let mut minute = 0;
    for (id, mins) in times {
        let time = mins.len();
        if time > sleepiest {
            guard = *id;
            sleepiest = time;
            let mut counts: HashMap<u8, usize> = HashMap::new();
            for min in mins {
                *counts.entry(*min).or_insert(0) += 1;
            }
            let mut highest = 0;
            for (m, no) in counts {
                if no > highest {
                    highest = no;
                    minute = m;
                }
            }
        }
    }
    (guard, minute)
}

/// Finds the guard that is most consistently asleep.
pub fn find_consistent<S: ::std::hash::BuildHasher>(
    times: &HashMap<GuardId, Vec<u8>, S>,
) -> (GuardId, u8) {
    let mut cons = 0;
    let mut guard = 0;
    let mut minute = 0;
    for (id, mins) in times {
        let mut counts: HashMap<u8, usize> = HashMap::new();
        for min in mins {
            *counts.entry(*min).or_insert(0) += 1;
        }
        for (m, no) in counts {
            if no > cons {
                cons = no;
                minute = m;
                guard = *id;
            }
        }
    }
    (guard, minute)
}

/// Returns all minutes that each guard is asleep (with duplicates).
///
/// This methods requires that its input be sorted.
pub fn sleep_times(events: &[&str]) -> HashMap<GuardId, Vec<u8>> {
    let events = events.iter().filter(|line| !line.is_empty()).map(|entry| {
        let (date, action) = entry.split_at(18);
        let minute = date[15..17].parse::<u8>().unwrap();
        let mut words = action.split_whitespace();
        let action = match words.next().unwrap() {
            "Guard" => Action::Start(words.next().unwrap().replace('#', "").parse().unwrap()),
            "falls" => Action::Sleep,
            "wakes" => Action::Wake,
            _ => panic!("Unexpected event format."),
        };
        (minute, action)
    });
    let mut guard: Option<GuardId> = None;
    let mut nap_start_minute: u8 = 0;
    let mut sleeps: HashMap<GuardId, Vec<u8>> = HashMap::new();
    for (minute, action) in events {
        match action {
            Action::Start(id) => guard = Some(id),
            Action::Sleep => nap_start_minute = minute,
            Action::Wake => {
                let start = nap_start_minute;
                let mut vec = sleeps
                    .entry(guard.expect("No guard on duty to wake up. Ensure the input is sorted."))
                    .or_default();
                for min in start..minute {
                    vec.push(min);
                }
            }
        }
    }
    sleeps
}

/// Solve the puzzle using the input in `puzzles/4.txt`.
///
/// Solutions are printed to stdout.
pub fn solve() {
    let input = include_str!("../puzzles/4.txt");
    let mut events = input.lines().collect::<Vec<_>>();
    events.sort();
    let times = sleep_times(&events);
    let sleepy = find_sleepy(&times);
    let sleepy = u32::from(sleepy.0) * u32::from(sleepy.1);
    let consistent = find_consistent(&times);
    let consistent = u32::from(consistent.0) * u32::from(consistent.1);
    println!("Day four solutions: {}, {}", sleepy, consistent);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_input() {
        let input = "[1518-11-01 00:00] Guard #10 begins shift\n[1518-11-01 00:05] falls asleep\n[1518-11-01 00:25] wakes up\n[1518-11-01 00:30] falls asleep\n[1518-11-01 00:55] wakes up\n[1518-11-01 23:58] Guard #99 begins shift\n[1518-11-02 00:40] falls asleep\n[1518-11-02 00:50] wakes up\n[1518-11-03 00:05] Guard #10 begins shift\n[1518-11-03 00:24] falls asleep\n[1518-11-03 00:29] wakes up\n[1518-11-04 00:02] Guard #99 begins shift\n[1518-11-04 00:36] falls asleep\n[1518-11-04 00:46] wakes up\n[1518-11-05 00:03] Guard #99 begins shift\n[1518-11-05 00:45] falls asleep\n[1518-11-05 00:55] wakes up";
        let mut events = input.lines().collect::<Vec<_>>();
        events.sort();
        let times = sleep_times(&events);
        let sleepy = find_sleepy(&times);
        assert_eq!(sleepy.0 * sleepy.1 as u16, 240);
        let consistent = find_consistent(&times);
        assert_eq!(consistent.0 * consistent.1 as u16, 4455);
    }
}
