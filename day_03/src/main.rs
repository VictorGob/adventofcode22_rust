use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let filename: &str = "input_d3.txt";
    let mut count: u32 = 0;
    // Generate alphabet
    let raw_alpha = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let mut alpha: HashMap<char, u32> = HashMap::new();
    for (i, ltr) in raw_alpha.enumerate() {
        alpha.insert(ltr, 1 + (i as u32));
    }

    match read_lines(filename) {
        Ok(lines) => lines.for_each(|line| match line {
            Ok(rucksack) => {
                let (split_a, split_b) = rucksack.split_at(rucksack.len() / 2);
                for c in split_a.chars() {
                    if split_b.contains(c) {
                        count += alpha.get(&c).unwrap();
                        break;
                    }
                }
            }
            Err(err) => panic!("Unable to parse! {}", err),
        }),
        Err(p_err) => panic!("Error: {}", p_err),
    }
    println!("Count: {}", count);

    /* Part 2 */
    // From https://github.com/ZAZPRO/adventofcode-2022/blob/main/03/rust/src/bin/part_two.rs
    let mut total_sum: u32 = 0;
    let file = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    for chunk in lines.chunks(3) {
        // Transform strs into slices of bytes.
        let first_member = chunk[0].as_bytes();
        let second_member = chunk[1].as_bytes();
        let third_member = chunk[2].as_bytes();

        // Create a Set out of the first member.
        let mut a: HashSet<u8> = HashSet::new();
        for v in first_member {
            a.insert(*v);
        }
        // Create a Set out of the second member.
        let mut b: HashSet<u8> = HashSet::new();
        for v in second_member {
            b.insert(*v);
        }

        // Create a Set out of the third member.
        let mut c: HashSet<u8> = HashSet::new();
        for v in third_member {
            c.insert(*v);
        }

        // B&C intersection vector.
        let b_c_intersection: Vec<&u8> = b.intersection(&c).collect();
        // Create a Set out of the second and third intersection.
        let mut bc: HashSet<u8> = HashSet::new();
        for v in b_c_intersection {
            bc.insert(*v);
        }

        // Find intersection of three sets, that's our common item.
        for v in a.intersection(&bc) {
            // Calculate priority for each item and add it to the total sum.
            if *v > 96 {
                total_sum += u32::from(*v) - 96;
            } else {
                total_sum += u32::from(*v) - 38;
            }
        }
    }
    println!("Task 2: {}", total_sum)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    Ok(io::BufReader::new(match File::open(filename) {
        Ok(file) => file,
        Err(error) => panic!("Error while opening the file, error={}", error),
    })
    .lines())
}
