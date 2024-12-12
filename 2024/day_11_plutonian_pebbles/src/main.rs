use dashmap::DashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    let input_file = std::env::args()
        .nth(1)
        .expect("Usage: <program> <input file>");
    // default to 25
    let blink_target = std::env::args()
        .nth(2)
        .unwrap_or("25".to_string())
        .parse::<i32>()
        .unwrap();
    let input_str = std::fs::read_to_string(input_file).unwrap();

    let input = parse_input(&input_str);
    let result = blink_times(input, blink_target);
    println!(
        "After {} blinks, there are {} rocks",
        blink_target,
        result
            .iter()
            .map(|r| r.value().load(Ordering::Relaxed))
            .map(|x| x as u128)
            .sum::<u128>()
    );
}

fn parse_input(input: &str) -> DashMap<u64, AtomicUsize> {
    let rocks = DashMap::<u64, AtomicUsize>::new();

    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .for_each(|r| {
            rocks
                .entry(r)
                .or_insert_with(|| AtomicUsize::new(0))
                .fetch_add(1, Ordering::Relaxed);
        });

    rocks
}

fn blink_times(rock_counts: DashMap<u64, AtomicUsize>, times: i32) -> DashMap<u64, AtomicUsize> {
    let mut new_rock_counts = rock_counts;

    for _ in 0..times {
        new_rock_counts = blink(&new_rock_counts);
    }

    new_rock_counts
}

fn blink(rock_counts: &DashMap<u64, AtomicUsize>) -> DashMap<u64, AtomicUsize> {
    let new_rock_counts = DashMap::<u64, AtomicUsize>::new();

    rock_counts.iter().for_each(|entry| {
        let rock = *entry.key();
        let count = entry.value().load(Ordering::Relaxed);

        if rock == 0 {
            new_rock_counts
                .entry(1)
                .or_insert(AtomicUsize::new(0))
                .fetch_add(count, Ordering::Relaxed);
            return;
        }

        let rock_digits = (rock as f64).log10().floor() as u64 + 1;

        if rock_digits % 2 != 0 {
            new_rock_counts
                .entry(rock * 2024)
                .or_insert(AtomicUsize::new(0))
                .fetch_add(count, Ordering::Relaxed);
            return;
        }

        let half = rock_digits / 2;
        let divisor = 10u64.pow(half as u32);
        let left = rock / divisor;
        let right = rock % divisor;
        new_rock_counts
            .entry(left)
            .or_insert(AtomicUsize::new(0))
            .fetch_add(count, Ordering::Relaxed);
        new_rock_counts
            .entry(right)
            .or_insert(AtomicUsize::new(0))
            .fetch_add(count, Ordering::Relaxed);
    });

    new_rock_counts
}
