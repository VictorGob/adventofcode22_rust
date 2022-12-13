use std::collections::VecDeque;
use std::io;

fn main() -> Result<(), io::Error> {
    let filename = "input_d5.txt";
    let file = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();

    // See https://www.reddit.com/r/rust/comments/ad74ls/a_vector_made_of_vectors/
    let mut ship_p1: Vec<VecDeque<char>> = Vec::new();
    let mut ship_p2: Vec<VecDeque<char>> = Vec::new();
    let mut cloned_to_ship_p2: bool = false;

    for line in lines {
        // Detect ship's initial state
        if line.contains('[') {
            // Fill gaps with underscore: "[_]"
            let fill_line = line.replace("    ", " [_] ");
            let split_line: Vec<&str> = fill_line.split_whitespace().collect();

            // Initialize ship vector
            if ship_p1.is_empty() {
                for crt in split_line {
                    let initial_char = crt.chars().collect::<Vec<char>>()[1];
                    // Do not push underscore
                    if initial_char != '_' {
                        ship_p1.push(VecDeque::from([initial_char]));
                    } else {
                        ship_p1.push(VecDeque::new());
                    }
                }
                continue;
            }
            // Fill the ship vector
            let mut mut_ship = ship_p1.iter_mut();
            for crt in split_line {
                let current_stack = mut_ship.next().unwrap();
                let new_char = crt.chars().collect::<Vec<char>>()[1];
                // Do not push underscore
                if new_char != '_' {
                    current_stack.push_back(new_char);
                }
            }
        // Register movements
        } else if line.contains("move") {
            // Copy ship_p1 to p2
            if !cloned_to_ship_p2 {
                ship_p2 = ship_p1.to_vec();
                cloned_to_ship_p2 = true;
            }
            let split_line: Vec<&str> = line.split_whitespace().collect();
            let quantity: u32 = split_line[1].parse::<u32>().unwrap();
            let origin: u32 = split_line[3].parse::<u32>().unwrap() - 1;
            let destiny: u32 = split_line[5].parse::<u32>().unwrap() - 1;
            // Solve part 1
            {
                let mut crates_to_move: VecDeque<char> = VecDeque::new();
                {
                    // Get crates from origin
                    let origin_vector = ship_p1.get_mut(usize::try_from(origin).unwrap()).unwrap();
                    for _ in 0..(quantity) {
                        crates_to_move.push_back(origin_vector.pop_front().unwrap());
                    }
                }
                // Push crates to new location
                let dest_vector = ship_p1.get_mut(usize::try_from(destiny).unwrap()).unwrap();
                for n in 0..(quantity) {
                    dest_vector.push_front(crates_to_move[n as usize])
                }
            }
            // Solve part 2
            {
                let mut crates_to_move: VecDeque<char> = VecDeque::new();
                {
                    // Get crates from origin
                    let origin_vector = ship_p2.get_mut(usize::try_from(origin).unwrap()).unwrap();
                    for _ in 0..(quantity) {
                        crates_to_move.push_front(origin_vector.pop_front().unwrap());
                    }
                }
                // Push crates to new location
                let dest_vector = ship_p2.get_mut(usize::try_from(destiny).unwrap()).unwrap();
                for n in 0..(quantity) {
                    dest_vector.push_front(crates_to_move[n as usize])
                }
            }
        } else {
            continue;
        }
    }
    // Get the first element of each vector
    let resp_1: String = ship_p1.iter().map(|x| x[0]).into_iter().collect();
    let resp_2: String = ship_p2.iter().map(|x| x[0]).into_iter().collect();
    println!();
    println!("Part 1: {}", resp_1);
    println!("Part 2: {}", resp_2);
    Ok(())
}
