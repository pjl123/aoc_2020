use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_14\\docking_data\\input.txt")
                    .expect("Something went wrong reading the file");

    let mut mask: Vec<char> = Vec::with_capacity(0);
    let mut mem: HashMap<usize, usize> = HashMap::new();

    for line in contents.split("\r\n") {
        if is_mask(line) {
            mask = line.chars().skip(7).collect();
            continue;
        }

        let ass = parse_assignment(line);
        let masked_addresses: Vec<usize> = apply_mask(convert_to_bin_array(ass.0), &mask)
                                            .iter()
                                            .map(|addr| convert_to_int(*addr))
                                            .collect();

        for addr in masked_addresses {
            let value = mem.entry(addr)
                            .or_insert(ass.1);
            *value = ass.1;
        }
        //println!("mem: {:?}", mem);
    }

    let mut total: usize = 0;
    for key in mem.keys() {
        total += mem[key];
    }
    println!("total mem values: {}", total);
}

fn is_mask(line: &str) -> bool {
    return line.contains("mask = ");
}

fn parse_assignment(ass_def: &str) -> (usize, usize) {
    let reg = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
    let mut mem_add: usize = 0;
    let mut val: usize = 0;
    for cap in reg.captures(ass_def) {
        mem_add = cap[1].parse().unwrap();
        val = cap[2].parse().unwrap();
    }
    return (mem_add, val);
}

fn convert_to_bin_array(val: usize) -> [i8; 36] {
    let bin_vals: Vec<i8> = format!("{:0>36b}", val).chars()
                                                    .map(|c| if c == '1' {1} else {0})
                                                    .collect();
    let mut ret: [i8; 36] = [0; 36];
    for i in 0..ret.len() {
        ret[i] = bin_vals[i];
    }
    return ret
}

fn apply_mask(bin_array: [i8; 36], mask_chars: &Vec<char>) -> Vec<[i8; 36]>{
    let mut new_bin_array: [i8; 36] = [0; 36];
    new_bin_array.copy_from_slice(&bin_array[0..]);

    let mut addresses: Vec<[i8; 36]> = Vec::with_capacity(1024);
    addresses.push(new_bin_array);
    for i in 0..mask_chars.len() {
        if mask_chars[i] == '0' {
            continue;
        }

        if mask_chars[i] == '1' {
            for j in 0..addresses.len() {
                addresses[j][i] = 1;
            }
        }

        if mask_chars[i] == 'X' {
            let mut new_addresses: Vec<[i8; 36]> = Vec::with_capacity(addresses.len());
            for j in 0..addresses.len() {
                let mut new_address: [i8; 36] = [0; 36];
                new_address.copy_from_slice(&addresses[j][0..]);
                addresses[j][i] = 0;
                new_address[i] = 1;
                new_addresses.push(new_address);
            }
            addresses.extend(new_addresses);
        }
    }
    return addresses;
}

fn convert_to_int(bin_array: [i8; 36]) -> usize {
    let bin_str: String = bin_array.iter()
                                   .map(|b| format!("{}", b))
                                   .collect();
    return usize::from_str_radix(bin_str.as_str(), 2).unwrap();
}