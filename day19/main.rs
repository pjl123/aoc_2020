use std::fs;
use regex::Regex;

#[derive(Debug, Clone)]
struct Rule {
    values: Vec<Vec<Value>>
}

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Constant (String),
    Reference (usize)
}

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_19\\monster_messages\\input.txt")
                    .expect("Something went wrong reading the file");

    let content_split: Vec<&str> = contents.split("\r\n\r\n").collect();
    let rule_split: Vec<&str> = content_split[0].split("\r\n").collect();
    let mut rules: Vec<Rule> = vec![Rule {values: Vec::new()}; rule_split.len()];
    for rule_def in rule_split.iter() {
        let (id, rule) = parse_rule(rule_def);
        rules[id] = rule;
    }

    while rules.iter().any(|rule| has_references(rule)) {
        for i in 0..rules.len() {
            if has_references(&rules[i]) {
                let new_rule: Rule = resolve_references(&rules[i], &rules);
                rules[i] = new_rule;
            }
        }
    }

    let rule_0: String = format!("^{}$", combine_constants(&rules[0]));
    let rule_0_reg = Regex::new(rule_0.as_str()).unwrap();
    println!("final regex: {}", rule_0_reg);
    let valid_count: usize = content_split[1].split("\r\n")
                                             .filter(|message| rule_0_reg.is_match(message))
                                             .count();
    println!("valid message count: {}", valid_count);
}

fn parse_rule(rule_def: &str) -> (usize, Rule) {
    let rule_split: Vec<&str> = rule_def.split(": ").collect();
    let id: usize = rule_split[0].parse().unwrap();

    let mut subsets: Vec<Vec<Value>> = Vec::with_capacity(2);
    for subset_def in rule_split[1].split(" | ") {
        let mut subset: Vec<Value> = Vec::new();
        for val_def in subset_def.split(" ") {
            if val_def.contains("\"") {
                subset.push(Value::Constant (val_def.replace("\"", "")));
            }
            else {
                let reference: usize = val_def.parse().unwrap();
                subset.push(Value::Reference (reference));
            }
        }
        subsets.push(subset);
    }
    return (id, Rule { values: subsets });
}

fn has_references(rule: &Rule) -> bool {
    for subset in rule.values.iter() {
        for value in subset.iter() {
            match value {
                Value::Constant (_) => continue,
                Value::Reference (_) => return true
            }
        }
    }
    return false;
}

fn resolve_references(rule: &Rule, rules: &Vec<Rule>) -> Rule {
    let mut new_subsets: Vec<Vec<Value>> = Vec::with_capacity(rule.values.len());
    for s in 0..rule.values.len() {
        let mut values: Vec<Value> = Vec::new();
        for v in 0..rule.values[s].len() {
            match &rule.values[s][v] {
                Value::Constant (constant) => values.push(Value::Constant (constant.clone())),
                Value::Reference (reference) => {
                    if !has_references(&rules[*reference]) {
                        values.push(Value::Constant (combine_constants(&rules[*reference])));
                    }
                    else {
                        values.push(Value::Reference (*reference));
                    }
                }
            }
        }
        new_subsets.push(values);
    }
    return Rule { values: new_subsets };
}

fn combine_constants(rule: &Rule) -> String {
    let consts: Vec<String> = rule.values.iter()
                                  .map(|values| {
                                      let combined: String = values.iter()
                                                                   .map(|val| {
                                                                       match val {
                                                                           Value::Constant (constant) => constant.clone(),
                                                                           Value::Reference (_) => String::from("")
                                                                        }
                                                                    })
                                                                    .collect();
                                      return combined;
                                  })
                                  .collect();
    return format!("({})", consts.join("|")) ;
}
