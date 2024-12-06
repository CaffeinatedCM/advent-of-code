use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: {} <input file>", args[0]);
        std::process::exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).unwrap();
    
    let print_queue = parse_input(&input);
    
    println!("Sum of valid middle page numbers: {}", sum_valid_middle_page_numbers(&print_queue));
    println!("Sum of fixed middle page numbers: {}", sum_fixed_middle_page_numbers(&print_queue));
}

#[derive(Debug)]
struct PrintQueue {
    rules: HashMap<i32, Vec<i32>>,
    orders: Vec<Vec<i32>>,
}

fn parse_input(input: &str) -> PrintQueue {
    let mut rules = HashMap::<i32, Vec<i32>>::new();
    let mut orders = Vec::new();
    let mut rules_done = false;

    for line in input.lines() {
        if line.is_empty() {
            rules_done = true;
            continue;
        }
        if !rules_done {
            let mut parts = line.split("|");
            let first = parts.next().unwrap();
            let second = parts.next().unwrap();

            let first_num = i32::from_str(first).unwrap();
            let second_num = i32::from_str(second).unwrap();
            if rules.contains_key(&first_num) {
                rules.get_mut(&first_num).unwrap().push(second_num);
            } else {
                rules.insert(first_num, vec![second_num]);
            }
            continue;
        } 

        let parts = line.split(",");
        let mut order = Vec::new();
        for part in parts {
            order.push(i32::from_str(part).unwrap());
        }
        orders.push(order);
    }

    PrintQueue { rules, orders }
}

fn sum_valid_middle_page_numbers(print_queue: &PrintQueue) -> i32 {
    let mut sum = 0;
    for order in print_queue.orders.iter() {
        if is_valid_print_order(&print_queue.rules, order) {
            sum += get_middle_page_number(order);
        }
    }

    sum
}

fn sum_fixed_middle_page_numbers(print_queue: &PrintQueue) -> i32 {
    let mut sum = 0;
    for order in print_queue.orders.iter() {
        if !is_valid_print_order(&print_queue.rules, order) {
            let fixed_order = fix_order(&print_queue.rules, order);
            sum += get_middle_page_number(&fixed_order);
        }
    }

    sum
}

fn is_valid_print_order(rules: &HashMap<i32, Vec<i32>>, order: &Vec<i32>) -> bool {
    for idx in (0..order.len()).rev() {
        let num = order[idx];
        match rules.get(&num) {
            Some(must_be_before) => {
                for before in must_be_before {
                    for i in 0..idx {
                        if order[i] == *before {
                            return false;
                        }
                    }
                }
            }
            None => {
                continue;
            }
        }
    }
    
    true
}

fn get_middle_page_number(order: &Vec<i32>) -> i32 {
    return order[order.len() / 2];
}

fn fix_order(rules: &HashMap<i32, Vec<i32>>, order: &Vec<i32>) -> Vec<i32> {
    let mut new_order = order.clone();

    while !is_valid_print_order(rules, &new_order) {
        for idx in (0..new_order.len()).rev() {
            let num = new_order[idx];
            match rules.get(&num) {
                Some(must_be_before) => {
                    for before in must_be_before {
                        for i in 0..idx {
                            if new_order[i] == *before {
                                new_order.swap(i, idx);
                            }
                        }
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }


    new_order
}
