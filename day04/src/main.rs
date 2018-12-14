extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    //let input = include_str!("../data/test.txt");
    let input = include_str!("../data/day04.txt");
    let mut data: Vec<(chrono::DateTime<chrono::Utc>, _)> = Vec::new();
    let data_re = Regex::new(r"\[(\d+)\-(\d+)\-(\d+)\s(\d+):(\d+)\]\s(.*)").unwrap();
    for line in input.lines() {
        for cap in data_re.captures_iter(&line) {
            let parse: Vec<u32> = vec![&cap[1],&cap[2],&cap[3],&cap[4],&cap[5]]
                                    .into_iter()
                                    .map(|x| x.parse().unwrap())
                                    .collect();
            
            let year = parse[0] as i32;
            let month = parse[1];
            let day = parse[2];
            let hour = parse[3];
            let min = parse[4];
            let action = String::from(&cap[6]);
            
            let dt = Utc.ymd(year,month,day).and_hms(hour,min,0);
            data.push((dt, action));
        }
    }
    // Sort by datetime
    data.sort_unstable_by(|a,b| a.0.cmp(&b.0));

    let guard_id_re = Regex::new(r".*#(\d+).*").unwrap();
    let mut guards = HashMap::new();
    let mut curr_guard = 0;
    let mut start_sleep = 90;
    for x in data {
        let dt = x.0;
        let action = x.1;

        match action.chars().next().unwrap() {
            'G' => {
                // Set guard on duty
                for cap in guard_id_re.captures_iter(&action) {
                    let guard_id: i32 = cap[1].parse().unwrap();
                    guards.entry(guard_id).or_insert([0; 60]);
                    curr_guard = guard_id;
                }
            },
            'w' => {
                // Record sleep activiy of current guard
                let mut sleep_sched = *guards.get(&curr_guard).unwrap();
                for i in start_sleep as usize..dt.minute() as usize {
                    sleep_sched[i] += 1;
                }
                guards.insert(curr_guard, sleep_sched);
            },
            'f' => {
                start_sleep = dt.minute();
            },
            _ => { println!("Panic!"); }
        }
    }

    // Find the guard that sleeps the most
    let mut most_hours_slept = 0;
    for (guard_id, sleep_sched) in &guards {
        let hours_slept = sleep_sched.iter().sum();
        if hours_slept > most_hours_slept {
            most_hours_slept = hours_slept;
            curr_guard = *guard_id;
        }
    }

    // Find the minute the guard sleep the most on
    let sleep_sched = guards.get(&curr_guard).unwrap();
    let mut most_slept_min = 0;
    let mut part1 = 0;
    for (i,n) in sleep_sched.iter().enumerate() {
        if n > &most_slept_min {
            most_slept_min = *n;
            part1 = curr_guard * i as i32;
        }
    }
    println!("Part 1: {}", part1);

    // Part 2:
    let mut most_freq_min = 0;
    let mut most_freq_ind = 0;
    for (guard_id, sleep_sched) in &guards {
        for (i,n) in sleep_sched.iter().enumerate() {
            if n > &most_freq_min {
                most_freq_min = *n;
                most_freq_ind = i as i32;
                curr_guard = *guard_id;
            }
        }
    }
    println!("Part 2: {}", most_freq_ind * curr_guard);
}