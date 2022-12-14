use itertools::Itertools;
use std::collections::VecDeque;
use std::io;

fn get_marker(line: &str) -> usize {
    // Initialize the packet with the first four characters
    let mut packet: VecDeque<char> = line.chars().into_iter().take(4).collect();
    for (pos, chr) in line.char_indices().skip(4) {
        // Check repeated chars
        let unique_packet_len = packet.iter().unique().count();
        if unique_packet_len < packet.len() {
            packet.remove(0);
            packet.push_back(chr);
        } else {
            return pos;
        }
    }
    0
}
fn get_msg(line: &str) -> usize {
    // Initialize the packet with the first 14 characters
    let mut packet: VecDeque<char> = line.chars().into_iter().take(14).collect();
    for (pos, chr) in line.char_indices().skip(14) {
        // Check repeated chars
        let unique_packet_len = packet.iter().unique().count();
        if unique_packet_len < packet.len() {
            packet.remove(0);
            packet.push_back(chr);
        } else {
            return pos;
        }
    }
    0
}

fn main() -> Result<(), io::Error> {
    let filename = "input_d6.txt";
    let file = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();

    for line in lines {
        println!("Task 1: {}", get_marker(line));
        println!("Task 2: {}", get_msg(line));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_marker() {
        let input_list = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ];
        for input in input_list {
            assert!(get_marker(input.0) == input.1)
        }
    }
    #[test]
    fn test_get_msg() {
        let input_list = [
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ];
        for input in input_list {
            assert!(get_msg(input.0) == input.1)
        }
    }
}
