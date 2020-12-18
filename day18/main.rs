use std::fs;
use regex::Regex;

enum Operation {
    Addition (usize, usize),
    Multiplication (usize, usize)
}

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_18\\operation_order\\input.txt")
                    .expect("Something went wrong reading the file");

    let sum: usize = contents.split("\r\n")
                             .map(|exp| evaluate_expression(exp))
                             .sum();

    println!("value: {}", sum);
}

fn evaluate_expression(exp_def: &str) -> usize {
    let mut expression: String = exp_def.to_owned();
    let mut paren_indices = find_parens(&expression);
    while paren_indices.1 > 0 {
        let paren_exp = &expression[paren_indices.0..paren_indices.1 + 1];
        let result = evaluate_simple_expression(&paren_exp[1..paren_exp.len() - 1]);
        expression = expression.replacen(paren_exp, &result, 1);
        paren_indices = find_parens(&expression);
    }

    expression = evaluate_simple_expression(&expression);
    let val: usize = expression.parse().unwrap();
    return val;
}

fn find_parens(exp_def: &str) -> (usize, usize) {
    let mut paren_stack: Vec<usize> = Vec::with_capacity(exp_def.len());

    let exp_chars: Vec<char> = exp_def.chars().collect();
    for i in 0..exp_chars.len() {
        if exp_chars[i] == '(' {
            paren_stack.push(i);
        }
        else if exp_chars[i] == ')' {
            let open_index: usize = paren_stack.pop().unwrap();
            return (open_index, i);
        }
    }
    return (0, 0);
}

fn evaluate_simple_expression(exp_def: &str) -> String {
    let add_reg = Regex::new(r"(\d+) (\+) (\d+)").unwrap();
    let mut expression: String = exp_def.to_owned();
    let mut op_match = add_reg.find(&expression);
    while op_match.is_some() {
        let match_val: &str = op_match.unwrap().as_str();
        let op = parse_expression(match_val, &add_reg);
        let value: String = evaluate_operation(op).to_string();
        expression = expression.replacen(match_val, &value, 1);
        op_match = add_reg.find(&expression);
    }

    let mul_reg = Regex::new(r"(\d+) (\*) (\d+)").unwrap();
    op_match = mul_reg.find(&expression);
    while op_match.is_some() {
        let match_val: &str = op_match.unwrap().as_str();
        let op = parse_expression(match_val, &mul_reg);
        let value: String = evaluate_operation(op).to_string();
        expression = expression.replacen(match_val, &value, 1);
        op_match = mul_reg.find(&expression);
    }
    return expression;
}

fn parse_expression(exp_def: &str, reg: &Regex) -> Operation {
    let mut operand1: usize = 0;
    let mut operand2: usize = 0;
    let mut operator: String = String::from("");
    for cap in reg.captures(exp_def) {
        operand1 = cap[1].parse().unwrap();
        operand2 = cap[3].parse().unwrap();
        operator = cap[2].to_owned();
    }

    if operator == String::from("+") {
        return Operation::Addition(operand1, operand2);
    }

    return Operation::Multiplication(operand1, operand2);
}

fn evaluate_operation(operation: Operation) -> usize {
    match operation {
        Operation::Addition (value1, value2) => value1 + value2,
        Operation::Multiplication (value1, value2) => value1 * value2
    }
}