use std::io;

fn main() -> Result<(), io::Error> {
    let filename = "input_d5.txt";
    let file = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();

    let mut ship: Vec<Vec<&str>> = Vec::new();
    for line in lines {
        if !line.contains('[') {
            break;
        }
        let fill_line = line.replace("    ", " [_] ");
        // println!("{}", fill_line);
        let split_line: Vec<&str> = fill_line.split_whitespace().collect();
        // println!("{:?}", split_line);
        // println!("{}", ship.len());
        
        if ship.is_empty() {
            // Initialize ship vector
            for crt in split_line {
                println!("{}", crt.chars().collect::<Vec<char>>()[1]);
            }

        }
    }
    Ok(())
}
