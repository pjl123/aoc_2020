use std::fs;
const MAGIC_SUM: u32 = 2020;

fn main() {
    let contents = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_1\\report_repair\\input.txt")
                    .expect("Something went wrong reading the file");

    let numbers: Vec<u32> = contents.split('\n').map(|line| line.trim().parse().unwrap()).collect();
    let found_2_nums: (u32, u32) = find_2_numbers(&numbers);
    println!("{} * {} = {}", found_2_nums.0, found_2_nums.1, found_2_nums.0 * found_2_nums.1);

    let found_3_nums: (u32, u32, u32) = find_3_numbers(&numbers);
    println!("{} * {} * {} = {}", found_3_nums.0, found_3_nums.1, found_3_nums.2, found_3_nums.0 * found_3_nums.1 * found_3_nums.2);
}

fn find_2_numbers(numbers: &Vec<u32>) -> (u32, u32) {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i == j {
                continue;
            }
            else if numbers[i] + numbers[j] == MAGIC_SUM {
                return (numbers[i], numbers[j]);
            }
        }
    }
    return (0, 0);
}

fn find_3_numbers(numbers: &Vec<u32>) -> (u32, u32, u32) {
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            for k in 0..numbers.len() {
                if i == j || i == k || j == k {
                    continue;
                }
                else if numbers[i] + numbers[j] + numbers[k] == MAGIC_SUM {
                    return (numbers[i], numbers[j], numbers[k]);
                }
            }
            
        }
    }
    return (0, 0, 0);
}