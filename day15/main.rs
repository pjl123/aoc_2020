use std::fs;
use std::collections::HashMap;

const TARGET: usize = 30000000;

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_15\\rambunctious_recitation\\input.txt")
                    .expect("Something went wrong reading the file");

    let (start_num, mut num_tracker) = init_game(contents.trim());

    let mut curr_spoken: usize = start_num;
    let mut index: usize = num_tracker.len() + 1;
    while index < TARGET {
        let last_spoken = num_tracker.entry(curr_spoken).or_insert(index);
        curr_spoken = index - *last_spoken;
        *last_spoken = index;
        index += 1;
    }
    println!("{}th spoken num: {}", TARGET, curr_spoken);
}

fn init_game(starting_nums: &str) -> (usize, HashMap<usize, usize>) {
    let mut num_tracker = HashMap::new();
    let starting_nums_iter: Vec<usize> = starting_nums.split(",")
                                                       .map(|num| {
                                                           let val: usize = num.parse().unwrap();
                                                           return val;
                                                       }).collect();
    for i in 0..starting_nums_iter.len() - 1 {
        num_tracker.insert(starting_nums_iter[i], i + 1);
    }
    return (starting_nums_iter[starting_nums_iter.len() - 1], num_tracker);
}