use std::fs;

#[derive(Debug, Clone, PartialEq)]
enum Status {
    Floor,
    Filled,
    Empty
}

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_11\\seating_system\\input.txt")
                    .expect("Something went wrong reading the file");

    let mut curr_state: Vec<Vec<Status>> = contents.split("\r\n")
                                          .map(|row| parse_row(row))
                                          .collect();

    let mut new_state = curr_state.clone();

    let mut changed: bool = true;
    while changed {
        changed = false;
        for i in 0..curr_state.len() {
            for j in 0..curr_state[i].len() {
                match curr_state[i][j] {
                    Status::Floor => new_state[i][j] = Status::Floor,
                    Status::Filled => {
                        let mut occupied = 0;
                        if i > 0 {
                            for row in (0..i).rev() {
                                match curr_state[row][j] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }                        

                        if i > 0 {
                            for row in (0..i).rev() {
                                if (i - row) > j {
                                    break;
                                }

                                match curr_state[row][j - (i - row)] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if j > 0 {
                            for col in (0..j).rev() {
                                match curr_state[i][col] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }
                        
                        if i < curr_state.len() - 1 {
                            for row in i + 1..curr_state.len() {
                                if (row - i) > j {
                                    break;
                                }

                                match curr_state[row][j - (row - i)] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }                        

                        if i < curr_state.len() - 1 {
                            for row in i + 1..curr_state.len() {
                                match curr_state[row][j] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if i < curr_state.len() - 1 {
                            for row in i + 1..curr_state.len() {
                                if j + (row - i) >= curr_state[i].len() {
                                    break;
                                }

                                match curr_state[row][j + (row - i)] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if j < curr_state[i].len() - 1 {
                            for col in j + 1..curr_state[i].len() {
                                match curr_state[i][col] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if i > 0 {
                            for row in (0..i).rev() {
                                if j + (i - row) >= curr_state[i].len() {
                                    break;
                                }

                                match curr_state[row][j + (i - row)] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if occupied >= 5 {
                            new_state[i][j] = Status::Empty;
                            changed = true;
                        }
                        else {
                            new_state[i][j] = Status::Filled;
                        }
                    },
                    Status::Empty => {
                        let mut occupied = 0;
                        if i > 0 {
                            for row in (0..i).rev() {
                                match curr_state[row][j] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }                        

                        if i > 0 {
                            for row in (0..i).rev() {
                                if (i - row) > j {
                                    break;
                                }

                                match curr_state[row][j - (i - row)] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if j > 0 {
                            for col in (0..j).rev() {
                                match curr_state[i][col] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }
                        
                        if i < curr_state.len() - 1 {
                            for row in i + 1..curr_state.len() {
                                if (row - i) > j {
                                    break;
                                }

                                match curr_state[row][j - (row - i)] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }                        

                        if i < curr_state.len() - 1 {
                            for row in i + 1..curr_state.len() {
                                match curr_state[row][j] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if i < curr_state.len() - 1 {
                            for row in i + 1..curr_state.len() {
                                if j + (row - i) >= curr_state[i].len() {
                                    break;
                                }

                                match curr_state[row][j + (row - i)] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if j < curr_state[i].len() - 1 {
                            for col in j + 1..curr_state[i].len() {
                                match curr_state[i][col] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if i > 0 {
                            for row in (0..i).rev() {
                                if j + (i - row) >= curr_state[i].len() {
                                    break;
                                }

                                match curr_state[row][j + (i - row)] {
                                    Status::Floor => continue,
                                    Status::Empty => break,
                                    Status::Filled => {
                                        occupied += 1;
                                        break;
                                    },
                                }
                            }
                        }

                        if occupied == 0 {
                            new_state[i][j] = Status::Filled;
                            changed = true;
                        }
                        else {
                            new_state[i][j] = Status::Empty;
                        }
                    }
                }
            }
        }
        curr_state = new_state.clone();
    }
    let count_filled = curr_state.iter()
                                 .map(|row| {
                                    row.iter()
                                       .filter(|col| **col == Status::Filled)
                                       .count()
                                 })
                                 .fold(0, |acc, row_sum| acc + row_sum);
    println!("count filled: {}", count_filled);
}

fn parse_row(row_def: &str) -> Vec<Status> {
    return row_def.chars()
                  .map(|col| {
                      if col == '.' {
                          return Status::Floor
                      }
                      else {
                          return Status::Empty
                      }
                  })
                  .collect();
}