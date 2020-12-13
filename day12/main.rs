use std::fs;
use regex::Regex;

#[derive(Debug)]
struct WayPointLocation {
    x: isize,
    y: isize
}

#[derive(Debug)]
struct Location {
    x: isize,
    y: isize
}

#[derive(Debug)]
enum Action {
    N (isize),
    S (isize),
    E (isize),
    W (isize),
    L (isize),
    R (isize),
    F (isize)
}

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_12\\rain_risk\\input.txt")
                    .expect("Something went wrong reading the file");
    
    let actions: Vec<Action> = contents.split("\r\n")
                                       .map(|line| parse_action(line))
                                       .collect();

    let mut waypoint_location = WayPointLocation {
        x: 10,
        y: -1
    };

    let mut location = Location {
        x: 0,
        y: 0
    };

    for action in actions {
        println!("action: {:?}", action);
        match action {
            Action::N (units) => waypoint_location.y -= units,
            Action::S (units) => waypoint_location.y += units,
            Action::E (units) => waypoint_location.x += units,
            Action::W (units) => waypoint_location.x -= units,
            Action::L (degrees) => {
                let rotation = (degrees / 90) % 4;
                if rotation == 1 {
                    let swp = waypoint_location.x;
                    waypoint_location.x = waypoint_location.y;
                    waypoint_location.y = -1 * swp;
                }
                else if rotation == 2 {
                    waypoint_location.x = -1 * waypoint_location.x;
                    waypoint_location.y = -1 * waypoint_location.y;
                }
                else if rotation == 3 {
                    let swp = waypoint_location.x;
                    waypoint_location.x = -1 * waypoint_location.y;
                    waypoint_location.y = swp;
                }
            },
            Action::R (degrees) => {
                let rotation = (degrees / 90) % 4;
                if rotation == 1 {
                    let swp = waypoint_location.x;
                    waypoint_location.x = -1 * waypoint_location.y;
                    waypoint_location.y = swp;
                }
                else if rotation == 2 {
                    waypoint_location.x = -1 * waypoint_location.x;
                    waypoint_location.y = -1 * waypoint_location.y;
                }
                else if rotation == 3 {
                    let swp = waypoint_location.x;
                    waypoint_location.x = waypoint_location.y;
                    waypoint_location.y = -1 * swp;
                }
            },
            Action::F (units) => {
                location.x += units * waypoint_location.x;
                location.y += units * waypoint_location.y;
            }
        }
        println!("waypoint location: {:?}", waypoint_location);
        println!("location: {:?}", location);
    }
    println!("Manhattan distance: {}", location.x.abs() + location.y.abs());
}

fn parse_action(def: &str) -> Action {
    let reg = Regex::new(r"^([NSEWLRF])(\d+)$").unwrap();

    let mut action: String = String::from("");
    let mut value: isize = 0;
    for cap in reg.captures(def) {
        action = String::from(&cap[1]);
        value = cap[2].parse().unwrap();
    }

    match action.as_str() {
        "N" => return Action::N (value),
        "S" => return Action::S (value),
        "E" => return Action::E (value),
        "W" => return Action::W (value),
        "L" => return Action::L (value),
        "R" => return Action::R (value),
        "F" => return Action::F (value),
        _ => return Action::L (0)
    }
}