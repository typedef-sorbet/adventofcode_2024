use std::{env::Args, io::BufReader};

/*
    The unusual data consists of many reports, one report per line. Each
    report is a list of numbers called "levels" that are separated by spaces.

    The engineers are trying to figure out which reports are "safe". The safety
    systems can only tolerate levels that are either gradually increasing or
    gradually decreasing. so, a report only counts as safe if both the following
    are true:

    - The levels are either all increasing, or all decreasing
    - Any two adjacent levels differ by at least one, and at most three

    For a given input, report the number of reports that are considered safe.
*/

static PUZZLE_INPUT: &str = include_str!("../../res/day_2.txt");

/*
    impl<'a, T> Iterator for Iter<'a, T>
    fn all<F>(&mut self, mut f: F) -> bool
    where
        Self: Sized,
        F: FnMut(Self::Item) -> bool,
*/

pub fn solution(_args: Args) -> Result<i32, &'static str> {
    let mut sum = 0;

    for report in PUZZLE_INPUT.lines() {
        let levels: Vec<i32> = report.split_ascii_whitespace()          // Split report on whitespace
                                    .map(|l| l.parse::<i32>())    // Parse each level as an i32
                                    .flatten().collect();               // Unwrap and collect

        let diffs: Vec<i32> = levels.iter()
                                .zip(&levels[1..])                  // Zip the levels vec such that we can iterate over tuples [1]
                                .map(|(first, next)|    // Compute the difference between the two
                                    first - next
                                )
                                .collect();                         // Collect into Vec

        // Part 1 Solution:
        // Ensure that diffs are ALL on EITHER [-3, 0) or (0, 3]
        if diffs.iter().all(|d| -3 <= *d && *d < 0) || diffs.iter().all(|d| 3 >= *d && *d > 0) {
            sum += 1;
        }
        else {
            // Part 2 Solution:
            // Check to see if removing any one of the levels would render the report safe
            // TODO there's probably a more efficient way to do this, but oh well

            for i in 0..levels.len() {
                let new_levels: Vec<i32> = [&levels[..i], &levels[i+1..]].concat();
                let new_devils: Vec<i32> = new_levels.iter()
                                            .zip(&new_levels[1..])
                                            .map(|(first, next)| first - next)
                                            .collect();

                if new_devils.iter().all(|d| -3 <= *d && *d < 0) || new_devils.iter().all(|d| 3 >= *d && *d > 0) {
                    sum += 1;
                    break;
                }
            }
        }
    }

    Ok(sum)
}

// [1]  This can be done easier with itertools::tuple_windows, but I want to avoid using outside crates
//      in the actual solutions, unless it's extremely difficult or non-ergonomic to do otherwise.
