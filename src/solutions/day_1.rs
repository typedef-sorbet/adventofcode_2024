use std::collections::HashMap;
use std::env::Args;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solution(_args: Args) -> Result<i32, &'static str> {
    let mut sum = 0i32;

    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();

    let mut count: HashMap<i32, i32> = HashMap::new();

    // Read the two lists from file
    let Ok(fd) = File::open("res/day_1.txt") else {
        return Err("Unable to open res/day_1.txt");
    };

    let bufreader = BufReader::new(fd);

    for line in bufreader.lines().flatten() {
        // The input is reliably 5 digit numbers, so this should be safe
        let trimmed_line = line.replace("   ", " ");
        let (elem_a, elem_b) = trimmed_line.split_at(5);

        list_a.push(elem_a.parse().unwrap());
        list_b.push(elem_b[1..].parse().unwrap());
    }

    // Build "similarity score list" before computing sum
    for i in 0..list_a.len() {
        let elem_b = list_b[i];

        if !count.contains_key(&elem_b) {
            count.insert(elem_b, 0);
        }

        *count.get_mut(&elem_b).unwrap() += 1;
    }

    // Sort both lists
    list_a.sort();
    list_b.sort();

    // Iterate over both simultaneously
    for i in 0..list_a.len() {
        // Part 1 solution
        // sum += list_a[i].abs_diff(list_b[i]) as i32;
        // Part 2 solution
        sum += list_a[i] * count.get(&list_a[i]).unwrap_or(&0i32);
    }

    // ...
    Ok(sum)
}