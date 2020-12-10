use std::fs;

struct Requirement {
    required_char: char,
    char_index_opt1: usize,
    char_index_opt2: usize
}

fn main() {
    let contents = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_2\\password_philosophy\\input.txt")
                    .expect("Something went wrong reading the file");
    
    let count_valid: usize = contents.split('\n') .map(|line| parse_line(line))
                                                .map(|(req, pass)| validate_password(req, pass))
                                                .filter(|is_valid| *is_valid)
                                                .count();
    println!("Found {} valid passwords!", count_valid);
}

fn parse_line(line: &str) -> (Requirement, &str) {
    let line_split: Vec<&str> = line.split(':').collect();
    let req: Requirement = parse_requirement(line_split[0].trim());
    let password: &str = line_split[1].trim();
    return (req, password);
}

fn parse_requirement(def: &str) -> Requirement {
    let req_split: Vec<&str> = def.split(' ').collect();
    let counts_split: Vec<&str> = req_split[0].split('-').collect();

    return Requirement {
        required_char: req_split[1].chars().nth(0).unwrap(),
        char_index_opt1: counts_split[0].trim().parse().unwrap(),
        char_index_opt2: counts_split[1].trim().parse().unwrap()
    };
}

fn validate_password(req: Requirement, password: &str) -> bool {
    let char_at_index1: bool = password.chars().nth(req.char_index_opt1 - 1).unwrap() == req.required_char;
    let char_at_index2: bool = password.chars().nth(req.char_index_opt2 - 1).unwrap() == req.required_char;
    
    return char_at_index1 ^ char_at_index2;
}