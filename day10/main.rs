use std::fs;

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_10\\adapter_array\\input.txt")
                    .expect("Something went wrong reading the file");

    let mut adapters: Vec<usize> = contents.split("\r\n")
                                       .map(|line| line.parse().unwrap())
                                       .collect();

    adapters.sort();
    
    let mut option_counts = Vec::with_capacity(adapters.len());
    for i in 0..adapters.len() {
        option_counts.push(get_count_options(i, &adapters));
        println!("options for {}: {}", adapters[i], option_counts[i]);
    }

    for i in (0..option_counts.len() - 1).rev() {
        if i + 1 < adapters.len() &&
            adapters[i + 1] - adapters[i] <= 3 {
            option_counts[i] = option_counts[i + 1];
        }

        if i + 2 < adapters.len() &&
            adapters[i + 2] - adapters[i] <= 3 {
            option_counts[i] += option_counts[i + 2];
        }

        if i + 3 < adapters.len() &&
            adapters[i + 3] - adapters[i] <= 3 {
            option_counts[i] += option_counts[i + 3];
        }
    }

    let mut count_configs: usize = 0;
    if adapters[0] <= 3 {
        count_configs += option_counts[0];
    }
    if adapters[1] <= 3 {
        count_configs += option_counts[1];
    }
    if adapters[2] <= 3 {
        count_configs += option_counts[2];
    }

    println!("total config options: {}", count_configs);
}

fn get_count_options(index: usize, adapters: &Vec<usize>) -> usize {
    if index == adapters.len() -1 {
        return 1;
    }

    let mut options = 0;
    if index + 1 < adapters.len() &&
        adapters[index + 1] - adapters[index] <= 3 {
        options += 1;
    }
    if index + 2 < adapters.len() &&
        adapters[index + 2] - adapters[index] <= 3 {
        options += 1;
    }
    if index + 3 < adapters.len() &&
        adapters[index + 3] - adapters[index] <= 3 {
        options += 1;
    }
    return options;
}
