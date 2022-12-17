use std::io;

/* See:
- https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
*/

/*#[derive(Debug)]
struct Folder<'a> {
    name: String,
    children: Vec<&'a str>,
    parent: &'a str,
    elements: Vec<&'a str>,
    size: u64,
} */

enum LineType {
    CdBack,
    CdEnter,
    Ls,
    FileInfo,
    DirInfo,
}

fn process_line(line: &str) -> (LineType, &str) {
    // Check order
    if line.starts_with('$') {
        // Check movement
        if line.contains("cd") {
            if line.contains("..") {
                (LineType::CdBack, "")
            } else {
                let folder = line
                    .split_whitespace()
                    .nth(2)
                    .expect("Error while parsing folder (accesing)");
                (LineType::CdEnter, folder)
            }
        } else if line.contains("ls") {
            (LineType::Ls, "")
        } else {
            todo!();
        }
    } else if line.starts_with("dir") {
        let dir = line
            .split_whitespace()
            .nth(1)
            .expect("Error while parsing folder (ls)");
        (LineType::DirInfo, dir)
    } else {
        let size = line
            .split_whitespace()
            .next()
            .expect("Error while parsing filesize");
        (LineType::FileInfo, size)
    }
}

fn main() -> Result<(), io::Error> {
    let filename = "input_d7.txt";
    let file = std::fs::read_to_string(filename).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    // let mut resp_1: Vec<Folder> = Vec::new();
    // let mut folder: HashMap<&str,u64> = HashMap::new();

    for line in lines {
        process_line(line);
    }
    println!("Task 1: {:?}", 1);

    Ok(())
}
