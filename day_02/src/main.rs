use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/*
- A for Rock, B for Paper, and C for Scissors
- X for Rock, Y for Paper, and Z for Scissors
- 1 for Rock, 2 for Paper, and 3 for Scissors
- 0 if you lost, 3 if the round was a draw, and 6 if you won
*/

fn main() {
    let filename: &str = "input_d2.txt";
    let mut count: i32 = 0;
    let mut count_p2: i32 = 0;
    let player_plays: HashMap<char, i32> = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);
    // hasmap tuple(opponent, player), value
    let round_plays: HashMap<(char, char), i32> = HashMap::from([
        (('A', 'X'), 3),
        (('A', 'Y'), 6),
        (('A', 'Z'), 0),
        (('B', 'X'), 0),
        (('B', 'Y'), 3),
        (('B', 'Z'), 6),
        (('C', 'X'), 6),
        (('C', 'Y'), 0),
        (('C', 'Z'), 3),
    ]);
    /*
    Part 2:
    - A for Rock, B for Paper, and C for Scissors
    - X for Rock, Y for Paper, and Z for Scissors
    - 1 for Rock, 2 for Paper, and 3 for Scissors
    */
    let plays_mapping: HashMap<(char, char), char> = HashMap::from([
        (('A', 'X'), 'Z'),
        (('A', 'Y'), 'X'),
        (('A', 'Z'), 'Y'),
        (('B', 'X'), 'X'),
        (('B', 'Y'), 'Y'),
        (('B', 'Z'), 'Z'),
        (('C', 'X'), 'Y'),
        (('C', 'Y'), 'Z'),
        (('C', 'Z'), 'X'),
    ]);

    match read_lines(filename) {
        Ok(lines) => {
            lines.for_each(|line| {
                match line {
                    Ok(round) => {
                        let r: Vec<char> = round
                            .trim()
                            .chars()
                            .filter(|x: &char| !x.is_whitespace())
                            .collect();
                        let oppo: char = r[0];
                        let player: char = r[1];

                        // count players hand and round result
                        count = count
                            + player_plays.get(&player).unwrap()
                            + round_plays.get(&(oppo, player)).unwrap();
                        // map player hand to the new mapping and calculate
                        let mapped_player: char = *plays_mapping.get(&(oppo, player)).unwrap();
                        count_p2 = count_p2
                            + player_plays.get(&mapped_player).unwrap()
                            + round_plays.get(&(oppo, mapped_player)).unwrap()
                    }
                    Err(err) => panic!("Unable to parse! {}", err),
                }
            });
        }
        Err(p_err) => panic!("Error: {}", p_err),
    }
    println!("Final count, part 1: {}", count);
    println!("Final count, part 2: {}", count_p2);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file: File = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("Error while opening the file, error={}", error),
    };
    Ok(io::BufReader::new(file).lines())
}
