use std::fs;

const UNIVERSE_WIDTH: usize = 26;
const UNIVERSE_MID_INDEX: usize = UNIVERSE_WIDTH / 2;
const INIT_WIDTH: usize = 8;

fn main() {
    let contents: String = fs::read_to_string("C:\\Users\\lafat\\Documents\\projects\\advent_of_code\\day_17\\conway_cubes\\input.txt")
                    .expect("Something went wrong reading the file");

    let init_state = parse_init_state(contents);

    let mut universe: [[[[bool; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH] = create_from_init(init_state);
    let mut new_universe: [[[[bool; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH];UNIVERSE_WIDTH] = [[[[false; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH];

    // println!("z = {}", UNIVERSE_MID_INDEX);
    // for y in 0..universe[UNIVERSE_MID_INDEX].len() {
    //     println!("{}:\t{:?}", y, universe[UNIVERSE_MID_INDEX][y]);
    // }

    let num_cycles: usize = 6;
    for _i in 0..num_cycles {
        for w in 0..universe.len() {
            for z in 0..universe[w].len() {
                for y in 0..universe[w][z].len() {
                    for x in 0..universe[w][z][y].len() {
                        let mut active_count: usize = 0;

                        let mut w_diff: usize = 0;
                        while w_diff <= 2 {
                            if (w_diff == 0 && w == 0) || (w_diff == 2 && w == UNIVERSE_WIDTH - 1){
                                w_diff += 1;
                                continue;
                            }

                            let w_to_check = if w_diff == 0 {w - 1} else {w + w_diff - 1};

                            let mut z_diff: usize = 0;
                            while z_diff <= 2 {
                                if (z_diff == 0 && z == 0) || (z_diff == 2 && z == UNIVERSE_WIDTH - 1){
                                    z_diff += 1;
                                    continue;
                                }

                                let z_to_check = if z_diff == 0 {z - 1} else {z + z_diff - 1};

                                let mut y_diff: usize = 0;
                                while y_diff <= 2 {
                                    if (y_diff == 0 && y == 0) || (y_diff == 2 && y == UNIVERSE_WIDTH - 1){
                                        y_diff += 1;
                                        continue;
                                    }
            
                                    let y_to_check = if y_diff == 0 {y - 1} else {y + y_diff - 1};

                                    let mut x_diff: usize = 0;
                                    while x_diff <= 2 {
                                        if w_diff == 1 && z_diff == 1 && y_diff == 1 && x_diff == 1 {
                                            x_diff += 1;
                                            continue;
                                        }

                                        if (x_diff == 0 && x == 0) || (x_diff == 2 && x == UNIVERSE_WIDTH - 1){
                                            x_diff += 1;
                                            continue;
                                        }

                                        let x_to_check = if x_diff == 0 {x - 1} else {x + x_diff - 1};
                                        

                                        if universe[w_to_check][z_to_check][y_to_check][x_to_check] {
                                            active_count += 1;
                                        }

                                        x_diff += 1;
                                    }
                                    y_diff += 1;
                                }
                                z_diff += 1;
                            }
                            w_diff += 1;
                        }

                        

                        if universe[w][z][y][x] {
                            new_universe[w][z][y][x] = active_count == 2 || active_count == 3;
                        }
                        else {
                            new_universe[w][z][y][x] = active_count == 3;
                        }
                        // if z == UNIVERSE_MID_INDEX {
                        //     println!("checking z={}, y={}, x={}: curr: {}, active_neighbors: {}, new: {}", z, y, x, universe[z][y][x], active_count, new_universe[z][y][x]);
                        // }
                    }
                }
            }
        }
        universe.clone_from_slice(&new_universe[0..]);
        // println!("z = {}", UNIVERSE_MID_INDEX);
        // for y in 0..universe[UNIVERSE_MID_INDEX].len() {
        //     println!("{}:\t{:?}", y, universe[UNIVERSE_MID_INDEX][y]);
        // }
    }

    let mut active_count: usize = 0;
    for w in universe.iter() {
        for z in w.iter() {
            for y in z.iter() {
                for x in y.iter() {
                    if *x {
                        active_count += 1;
                    }
                }
            }
        }
    }

    println!("remaining active: {}", active_count);
}

fn parse_init_state(def: String) -> [[bool; INIT_WIDTH]; INIT_WIDTH] {
    let mut init_state: [[bool; INIT_WIDTH]; INIT_WIDTH] = [[false; INIT_WIDTH]; INIT_WIDTH];
    let rows: Vec<&str> = def.split("\r\n").collect();
    for i in 0..rows.len() {
        let cols: Vec<char> = rows[i].chars().collect();
        for j in 0..cols.len() {
            if cols[j] == '#' {
                init_state[i][j] = true;
            }
        }
    }
    return init_state;
}

fn create_from_init(init_state: [[bool; INIT_WIDTH]; INIT_WIDTH]) -> [[[[bool; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH] {
    let mut universe: [[[[bool; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH] = [[[[false; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH]; UNIVERSE_WIDTH];

    for row_i in 0..init_state.len() {
        for col_i in 0..init_state[row_i].len() {
            universe[UNIVERSE_MID_INDEX][UNIVERSE_MID_INDEX][row_i + UNIVERSE_MID_INDEX][col_i + UNIVERSE_MID_INDEX] = init_state[row_i][col_i];
        }
    }

    return universe;
}