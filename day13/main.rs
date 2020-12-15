use std::{fs, thread};

const MAX_VAL: usize = 4200000000000000;
const START_VAL: usize = 347477892722236;
const NUM_THREADS: usize = 4;
static mut CANCELLED: bool = false;

fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        unsafe {
            CANCELLED = true;
        }
    }).expect("Unable to set ctrl-c!");

    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_13\\shuttle_search\\input.txt")
                    .expect("Something went wrong reading the file");

    let content_split: Vec<&str> = contents.split("\r\n").collect();
    let bus_ids: Vec<String> = content_split[1].split(",").map(|line| line.to_owned()).collect();
    let mut max_bus_index: usize = 0;
    for i in 0..bus_ids.len() {
        if bus_ids[i] == "x" {
            continue;
        }

        if bus_ids[i] > bus_ids[max_bus_index] {
            max_bus_index = i;
        }
    }

    let max_bus_interval: usize = bus_ids[max_bus_index].parse().unwrap();
    let start_val: usize = (START_VAL / max_bus_interval) * max_bus_interval;
    println!("start_val: {}", start_val);

    let mut handles = Vec::with_capacity(NUM_THREADS);
    for i in 0..NUM_THREADS {
        let bids = bus_ids.clone();
        let thread_interval: usize = max_bus_interval * NUM_THREADS;
        let thread_start_val: usize = (i * max_bus_interval) + start_val;
        println!("thread start: {}, thread_interval: {}", thread_start_val, thread_interval);
        handles.push(thread::spawn(move|| -> usize {
            unsafe {
                return test_bus_schedule(thread_start_val, MAX_VAL, &bids, thread_interval, max_bus_index);
            }
        }));
    }
    for handle in handles {
        let res: usize = handle.join().unwrap();
        if res != 0 {
            println!("found result: {}", res);
        }
    }
}

unsafe fn test_bus_schedule(min_val: usize, max_val: usize, bus_ids: &Vec<String>, interval: usize, base_bus_index: usize) -> usize{
    let mut init_depart: usize = min_val;
    while init_depart < max_val && !CANCELLED {
        let mut is_correct: bool = true;
        for j in 0..bus_ids.len() {
            if bus_ids[j] == "x" ||
                j == base_bus_index {
                continue;
            }
            
            let test_val: usize;
            if j < base_bus_index {
                test_val = init_depart - (base_bus_index - j);
            }
            else {
                test_val = init_depart + (j - base_bus_index);
            }

            let bus_interval: usize = bus_ids[j].parse().unwrap();
            if test_val % bus_interval != 0 {
                is_correct = false;
                break;
            }            
        }
        if is_correct {
            CANCELLED = true;
            return init_depart - base_bus_index;
        }

        init_depart += interval;
    }
    
    println!("max val checked: {}", init_depart);
    return init_depart - init_depart;
}