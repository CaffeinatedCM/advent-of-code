use std::env;

// main function get the input from the first arg of the command line
fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();
    parse_input(&input, &mut vec1, &mut vec2);

    let distance = find_difference_in_pairs(&mut vec1, &mut vec2);

    println!("Distance: {}", distance);
    
    let similarity = find_similarity_score(vec1, vec2);

    println!("Similarity: {}", similarity);
}

fn parse_input(input: &str, vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) {
    for line in input.lines() {
        let mut parts = line.split("   ");
        let first = parts.next().unwrap();
        let second = parts.next().unwrap();

        vec1.push(first.parse::<i32>().unwrap());
        vec2.push(second.parse::<i32>().unwrap());
    }
}

fn find_difference_in_pairs(vec1: &mut Vec<i32>, vec2: &mut Vec<i32>) -> i32 {
    let mut result = 0;
    vec1.sort();
    vec2.sort();

    for i in 0..vec1.len() {
        result += (vec1[i] - vec2[i]).abs();
    }

    result
}

fn find_similarity_score(vec1: Vec<i32>, vec2: Vec<i32>) -> i32 {
    let mut result = 0;

    for i in 0..vec1.len() {
        let mut count_in_vec2 = 0;
        for j in 0..vec2.len() {
            if vec1[i] == vec2[j] {
                count_in_vec2 += 1;
            }
        }

        result += vec1[i] * count_in_vec2;
    }

    result
}

