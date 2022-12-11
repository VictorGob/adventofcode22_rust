use std::io;

fn main() -> Result<(), io::Error> {
    let filename = "input_d5.txt";
    let file = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();

    // See https://www.reddit.com/r/rust/comments/ad74ls/a_vector_made_of_vectors/
    let mut ship: Vec<Vec<char>> = Vec::new();
    // Read the ship's stacks
    for line in lines {
        // Stop if no brackets are found
        if !line.contains('[') {
            break;
        }
        let fill_line = line.replace("    ", " [_] ");
        // println!("{}", fill_line);
        let split_line: Vec<&str> = fill_line.split_whitespace().collect();
        // println!("split_line: {:?}", split_line);
        // println!("{}", ship.len());

        if ship.is_empty() {
            // Initialize ship vector
            for crt in split_line {
                // println!("{}", crt.chars().collect::<Vec<char>>()[1]);
                let initial_char = crt.chars().collect::<Vec<char>>()[1];
                if initial_char != '_' {
                    ship.push(vec![initial_char]);
                } else {
                    ship.push(vec![]);
                }
            }
            continue;
        }

        let mut mut_ship = ship.iter_mut();
        for crt in split_line {
            let current_stack = mut_ship.next().unwrap();
            let new_char = crt.chars().collect::<Vec<char>>()[1];
            if new_char != '_' {
                current_stack.push(new_char);
            }
        }
    }
    for line in &ship {
        println!("{:?}", line);
    }
    Ok(())
}
