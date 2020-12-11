use std::fs;
use std::collections::HashMap;

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_6\\custom_customs\\input.txt")
                    .expect("Something went wrong reading the file");
    
    let count_sum: usize = contents.split("\r\n\r\n")
                                   .map(|group| convert_group_to_vec(group))
                                   .map(|group_vec| count_answers(group_vec))
                                   .fold(0, |acc, x| acc + x);
    println!("Count of sums: {}", count_sum);
}

fn count_answers(group: Vec<&str>) -> usize {
    let mut answers = HashMap::new();
    for member in &group {
        for answer in member.chars() {
            answers.entry(answer)
                   .and_modify(|e| *e += 1)
                   .or_insert(1);
        }
    }
    return answers.keys()
                  .filter(|key| (&group).len() == answers[key])
                  .count();
}

fn convert_group_to_vec(group_def: &str) -> Vec<&str> {
    return group_def.split("\r\n")
                    .map(|member| member.trim())
                    .collect();
}