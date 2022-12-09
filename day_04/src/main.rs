use std::io;

fn compare_elves(a: Vec<u32>, b: Vec<u32>) -> bool {
    a.first().unwrap() <= b.first().unwrap() && a.get(1).unwrap() >= b.get(1).unwrap()
}
fn main() -> Result<(), io::Error> {
    let filename = "input_d4.txt";
    let file = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let mut count_part1: u32 = 0;
    for line in lines {
        // Split the line in 2
        let pair: Vec<&str> = line.split(',').collect();
        // Split each part in values for each elve
        let elve_1: Vec<u32> = pair
            .first()
            .unwrap()
            .split('-')
            .map(|x| x.parse().expect("parse error"))
            .collect();
        let elve_2: Vec<u32> = pair
            .get(1)
            .unwrap()
            .split('-')
            .map(|x| x.parse().expect("parse error"))
            .collect();
            
        /* Part 1 */
        // Get the elve with a larger range of values, and pass it as the first
        let contains: bool = if (elve_1.get(1).unwrap() - elve_1.first().unwrap())
            > (elve_2.get(1).unwrap() - elve_2.first().unwrap())
        {
            compare_elves(elve_1, elve_2)
        } else {
            compare_elves(elve_2, elve_1)
        };
        if contains {
            count_part1 += 1;
        }

        /* Part 2 */
    }
    println!("Task 1 count: {}", count_part1);
    Ok(())
}
