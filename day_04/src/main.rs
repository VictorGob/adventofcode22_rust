use std::io;

fn compare_elves_part1(a: &[u32], b: &[u32]) -> bool {
    /*
    Elf "a" has a larger range of values. We compare if its extreme values go beyond
    the values of the elf with the lowest range (elf "b")
    */
    a.first().unwrap() <= b.first().unwrap() && a.get(1).unwrap() >= b.get(1).unwrap()
}

fn compare_elves_part2(a: &[u32], b: &[u32]) -> bool {
    /*
    Compare the lowest value of elf "a" with the highest of elf "b", and vice versa.
    If there is overlap, at least one of the extremes will not satisfy the condition
    */
    !(a.first().unwrap() > b.get(1).unwrap() || a.get(1).unwrap() < b.first().unwrap())
}

fn main() -> Result<(), io::Error> {
    let filename = "input_d4.txt";
    let file = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let mut count_part1: u32 = 0;
    let mut count_part2: u32 = 0;
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
            compare_elves_part1(&elve_1, &elve_2)
        } else {
            compare_elves_part1(&elve_2, &elve_1)
        };
        if contains {
            count_part1 += 1;
        }

        /* Part 2 */
        // Check for overlap
        let overlap: bool = compare_elves_part2(&elve_1, &elve_2);
        if overlap {
            count_part2 += 1;
        }
    }
    println!("Task 1 count: {}", count_part1);
    println!("Task 2 count: {}", count_part2);
    Ok(())
}
