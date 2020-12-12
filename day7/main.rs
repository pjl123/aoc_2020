use std::fs;
use std::collections::HashMap;
use regex::Regex;

#[derive(Clone)]
enum Bag {
    Leaf,
    Container (ContainerDef, ContainerDef, ContainerDef, ContainerDef)
}

#[derive(Clone, Debug)]
struct ContainerDef {
    content_key: String,
    quantity: usize
}

#[derive(Debug)]
struct BagNode {
    key: String,
    children: Vec<BagNode>
}

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_7\\handy_haversacks\\input.txt")
                    .expect("Something went wrong reading the file");
    
    let entries: Vec<(String, Bag)> = contents.split("\r\n")
                                              .map(|line| get_entry(line))
                                              .collect();
    
    let mut def_map = HashMap::new();
    for entry in entries {
        def_map.insert(entry.0, entry.1.clone());
    }

    let count_gold = count_bag_children(&def_map["shiny gold"], &def_map) - 1;
    println!("Count of gold: {}", count_gold);
}


fn get_entry(line: &str) -> (String, Bag) {
    let line_split: Vec<&str> = line.split(" bags contain ").collect();
    let key = line_split[0];

    let mut container_defs = Vec::with_capacity(4);
    let container_regex = Regex::new(r"\b(\d*? ?[a-z]+? [a-z]+?)\b bag").unwrap();
    for cap in container_regex.captures_iter(line_split[1]) {
        container_defs.push(get_container_def(cap[1].trim()))
    }

    if container_defs[0].quantity == 0 {
        return (String::from(key), Bag::Leaf);
    }

    return (String::from(key), Bag::Container (
        ContainerDef{
            content_key: String::from(container_defs[0].content_key.as_str()),
            quantity: container_defs[0].quantity
        },
        if container_defs.len() > 1 {
            ContainerDef{
                content_key: String::from(container_defs[1].content_key.as_str()),
                quantity: container_defs[1].quantity
            }
        }
        else {
            ContainerDef {
                content_key: String::from(""),
                quantity: 0
            }
        },
        if container_defs.len() > 2 {
            ContainerDef{
                content_key: String::from(container_defs[2].content_key.as_str()),
                quantity: container_defs[2].quantity
            }
        }
        else {
            ContainerDef {
                content_key: String::from(""),
                quantity: 0
            }
        },
        if container_defs.len() > 3 {
            ContainerDef{
                content_key: String::from(container_defs[3].content_key.as_str()),
                quantity: container_defs[3].quantity
            }
        }
        else {
            ContainerDef {
                content_key: String::from(""),
                quantity: 0
            }
        }
    ))
}

fn get_container_def(container_str: &str) -> ContainerDef {
    let content_key_regex = Regex::new(r"\b[a-z]+? [a-z]+?\b").unwrap();
    let content_key = content_key_regex.find(container_str).unwrap().as_str();

    let quantity: usize;
    if content_key == "no other" {
        quantity = 0;
    }
    else {
        let quantity_regex = Regex::new(r"\d+").unwrap();
        quantity = quantity_regex.find(container_str).unwrap().as_str().parse().unwrap();
    }

    return ContainerDef {
        content_key: String::from(content_key),
        quantity: quantity
    }
}

fn count_bag_children(bag: &Bag, bag_defs: &HashMap<String, Bag>) -> usize {
    match bag {
        Bag::Leaf => 
            return 0,
        Bag::Container (bag_1, bag_2, bag_3, bag_4) =>
            return get_bag_counts(bag_1, bag_2, bag_3, bag_4, bag_defs)
    }
}

fn get_bag_counts(bag_1: &ContainerDef, bag_2: &ContainerDef, bag_3: &ContainerDef, bag_4: &ContainerDef, bag_defs: &HashMap<String, Bag>) -> usize {
    return 1 + get_bag_count(bag_1, bag_defs) +
           get_bag_count(bag_2, bag_defs) +
           get_bag_count(bag_3, bag_defs) +
           get_bag_count(bag_4, bag_defs);
}

fn get_bag_count(bag: &ContainerDef, bag_defs: &HashMap<String, Bag>) -> usize{
    print!("getting count for: {:?}", bag);
    if bag.quantity == 0 {
        return 0;
    }
    let bag_child_count = count_bag_children(&bag_defs[&bag.content_key], bag_defs);
    if bag_child_count == 0 {
        println!("...{}", bag.quantity);
        return bag.quantity;
    }

    println!("...{}", bag.quantity * bag_child_count);
    return bag.quantity * bag_child_count;
}