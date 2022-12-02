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

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(round) = line {
                // println!("{}", round.trim());
                let r: Vec<char> = round
                    .trim()
                    .chars()
                    .filter(|x: &char| !x.is_whitespace())
                    .collect();
                let oppo = r[0];
                let player = r[1];

                // count players hand and round result
                count = count
                    + player_plays.get(&player).unwrap()
                    + round_plays.get(&(oppo, player)).unwrap();
            }
        }
    }
    println!("Final count:  {}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("Error while opening the file, error={}", error),
    };
    Ok(io::BufReader::new(file).lines())
}
