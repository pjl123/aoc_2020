use std::fs;

const TREE_DEF: char = '#';

fn main() {
    let contents = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_3\\toboggan_trajectory\\input.txt")
                    .expect("Something went wrong reading the file");
    
    let lines: Vec<&str> = contents.split('\n').map(|line| line.trim()).collect();
    let tree_product: usize = get_tree_counts(&lines, 1, 1) * 
                              get_tree_counts(&lines, 3, 1) *
                              get_tree_counts(&lines, 5, 1) *
                              get_tree_counts(&lines, 7, 1) *
                              get_tree_counts(&lines, 1, 2);

    println!("Tree product: {}", tree_product);
}

fn get_tree_counts(lines: &Vec<&str>, x_slope: usize, y_slope: usize) -> usize {
    let x_positions: Vec<usize> = get_x_positions(lines.len(), x_slope, y_slope);

    return x_positions.iter()
                      .zip(lines.iter())
                      .map(|(x_pos, line)| is_tree(x_pos, line))
                      .filter(|is_tree| *is_tree)
                      .count();
}

fn get_x_positions(max_y: usize, x_slope: usize, y_slope: usize) -> Vec<usize> {
    let mut x_positions = vec![0; max_y];
    for i in 0..max_y {
        if i % y_slope != 0 {
            continue;
        }

        x_positions[i] = ((i / y_slope) * x_slope) + 1;
    }
    return x_positions;
}

fn is_tree(x_pos: &usize, row: &str) -> bool {
    if *x_pos == 0 {
        return false;
    }

    let max_x: usize = row.len();
    let i: usize = if x_pos % max_x == 0 {max_x} else {x_pos % max_x};
    return row.chars().nth(i - 1).unwrap() == TREE_DEF;
}