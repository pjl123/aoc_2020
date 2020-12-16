use std::fs;
use std::collections::HashMap;

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_16\\ticket_translation\\input.txt")
                    .expect("Something went wrong reading the file");

    let content_split: Vec<&str> = contents.split("\r\n\r\n").collect();
    let rules: Vec<(&str, Vec<(usize, usize)>)> = content_split[0].split("\r\n")
                                                          .map(|rule_line| {
                                                              let (field, rule_def) = get_field_rule_def(rule_line);
                                                              return (field, parse_rule(rule_def));
                                                          })
                                                          .collect();

    let valid_nearby_tickets: Vec<&str> = content_split[2].split("\r\n")
                                                          .skip(1)
                                                          .filter(|ticket| {
                                                              return ticket.split(",")
                                                                           .all(|field| is_field_valid_all_rules(field, &rules));
                                                          })
                                                          .collect();

    let mut fields: Vec<Vec<usize>> = vec![Vec::with_capacity(valid_nearby_tickets.len()); rules.len()];
    for i in 0..valid_nearby_tickets.len() {
        let field_split: Vec<usize> = valid_nearby_tickets[i].split(",")
                                                            .map(|field| {
                                                                let field_val: usize = field.parse().unwrap();
                                                                return field_val;
                                                            })
                                                            .collect();

        for j in 0..field_split.len() {
            fields[j].push(field_split[j]);
        }
    }

    let mut possible_positions: HashMap<&str, Vec<usize>> = HashMap::new();

    for i in 0..rules.len() {
        for rule in rules.iter() {
            if all_fields_satisfy_rule(&fields[i], &rule.1) {
                possible_positions.entry(rule.0)
                                  .and_modify(|e| e.push(i))
                                  .or_insert(vec![i; 1]);
            }
        }
    }

    let mut field_order: Vec<&str> = vec![""; rules.len()];
    while field_order.iter().any(|field| field.len() == 0) {

        for rule in rules.iter() {
            let positions = possible_positions.entry(rule.0)
                                            .or_insert(Vec::new());

            if positions.len() == 1 {
                let position = positions[0];
                field_order[position] = rule.0;
                positions.clear();
                for rule in rules.iter() {
                    let pos = possible_positions.entry(rule.0)
                                                .or_insert(Vec::new());
                    for i in (0..pos.len()).rev() {
                        if pos[i] == position {
                            pos.remove(i);
                        }
                    }
                }
            }
        }
    }

    let my_ticket_fields: Vec<usize> = content_split[1].replace("your ticket:\r\n", "").split(",")
                                                       .map(|field| {
                                                            let field_val: usize = field.parse().unwrap();
                                                            return field_val;
                                                       })
                                                       .collect();

    let mut product: usize = 1;
    for i in 0..field_order.len() {
        if field_order[i].contains("departure") {
            product *= my_ticket_fields[i];
        }
    }

    println!("product: {}", product);
}

fn get_field_rule_def(def: &str) -> (&str, &str) {
    let def_split: Vec<&str> = def.split(": ").collect();
    return (def_split[0], def_split[1]);
}

fn parse_rule(def: &str) -> Vec<(usize, usize)>{
    let mut ranges: Vec<(usize, usize)> = Vec::with_capacity(2);
    for range in def.split(" or ") {
        let range_split: Vec<&str> = range.split("-").collect();
        let low_num: usize = range_split[0].parse().unwrap();
        let high_num: usize = range_split[1].parse().unwrap();
        ranges.push((low_num, high_num));
    }
    return ranges;
}

fn is_field_valid_all_rules(field: &str, rules: &Vec<(&str, Vec<(usize, usize)>)>) -> bool {
    for rule in rules {
        let field_val: usize = field.parse().unwrap();

        if is_field_valid_one_rule(field_val, &rule.1) {
            return true;
        }
    }
    return false;
}

fn is_field_valid_one_rule(field: usize, rule: &Vec<(usize, usize)>) -> bool {
    if field >= rule[0].0 && field <= rule[0].1 {
        return true;
    }

    if field >= rule[1].0 && field <= rule[1].1 {
        return true;
    }

    return false;
}

fn all_fields_satisfy_rule(fields: &Vec<usize>, rule: &Vec<(usize, usize)>) -> bool {
    return fields.iter().all(|field| is_field_valid_one_rule(*field, &rule));
}