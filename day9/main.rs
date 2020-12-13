use std::fs;

const PREAMBLE_SIZE: usize = 25;

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_9\\encoding_error\\input.txt")
                    .expect("Something went wrong reading the file");
    
    let data: Vec<usize> = contents.split("\r\n")
                                   .map(|line| line.parse().unwrap())
                                   .collect();
    
    let mut invalid_data: usize = 0;
    for i in PREAMBLE_SIZE..data.len() {
        if !validate_data(data.iter().skip(i - PREAMBLE_SIZE).take(PREAMBLE_SIZE).collect(), data[i]) {
            invalid_data = data[i];
            break;
        }
    }

    for i in 0..data.len() - 1 {
        let weakness = find_weakness(data.iter().skip(i).collect(), invalid_data);
        if weakness.0 {
            println!("found weakness: {}", weakness.1);
            break;
        }
    }
}

fn validate_data(preamble: Vec<&usize>, to_validate: usize) -> bool {
    for i in 0..preamble.len() {
        for j in 0..preamble.len() {
            if i == j {
                continue;
            }

            if preamble[i] + preamble[j] == to_validate {
                return true;
            }
        }
    }
    return false;
}

fn find_weakness(data: Vec<&usize>, target_sum: usize) -> (bool, usize) {
    let mut sum: usize = *data[0];
    let mut end: usize = 1;
    while sum < target_sum && end <= data.len() {
        sum += data[end];
        end += 1;
    }

    if sum != target_sum {
        return (false, 0);
    }

    let mut smallest = usize::MAX;
    let mut largest = 0;

    for i in 0..end - 1 {
        if *data[i] < smallest {
            smallest = *data[i];
        }

        if *data[i] > largest {
            largest = *data[i];
        }
    }

    return (true, smallest + largest);
}