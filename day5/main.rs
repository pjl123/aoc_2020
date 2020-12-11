use std::fs;
use regex::Regex;

const MAX_SEAT_ID: usize = 1024;
const ROW_BIN_1: char = 'B';
const ROW_BIN_0: char = 'F';
const COL_BIN_1: char = 'R';
const COL_BIN_0: char = 'L';

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_5\\binary_boarding\\input.txt")
                    .expect("Something went wrong reading the file");
    
    let mut seats: [u8; MAX_SEAT_ID] = [0; MAX_SEAT_ID];
    let seat_ids: Vec<usize> = contents.split('\n')
                                       .map(|line| get_row_and_column_def(line))
                                       .map(|row_and_col| (convert_bin_str_to_int(row_and_col.0), convert_bin_str_to_int(row_and_col.1)))
                                       .map(|seat_spec| get_seat_id(seat_spec))
                                       .collect();

    for seat_id in seat_ids {
        seats[seat_id] = 1;
    }

    let mut last_filled: bool = seats[1] == 1;
    let mut last_two_filled: bool = seats[2] == 1;
    for i in 2..seats.len() {
        if seats[i] == 1 &&
            last_filled == false &&
            last_two_filled == true {
                println!("Seat id: {}", i - 2);
                return;
            }
            last_filled = seats[i - 1] == 1;
            last_two_filled = seats[i - 2] == 1;
    }
}

fn get_row_and_column_def(seat_def: &str) -> (String, String) {
    let row_regex = Regex::new(r"[BF]{7}").unwrap();
    let row_def = row_regex.find(seat_def).unwrap()
                                          .as_str()
                                          .replace(ROW_BIN_1, "1")
                                          .replace(ROW_BIN_0, "0");

    let col_regex = Regex::new(r"[LR]{3}").unwrap();
    let col_def = col_regex.find(seat_def).unwrap()
                                          .as_str()
                                          .replace(COL_BIN_1, "1")
                                          .replace(COL_BIN_0, "0");

    return (row_def, col_def);
}

fn convert_bin_str_to_int(bin_str: String) -> usize {
    return usize::from_str_radix(bin_str.as_str(), 2).unwrap();
}

fn get_seat_id(seat_spec: (usize, usize)) -> usize {
    return seat_spec.0 * 8 + seat_spec.1;
}