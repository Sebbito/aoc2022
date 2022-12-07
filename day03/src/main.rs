use std::fs;
use itertools::Itertools;

const PATH_TO_FILE: &str = "input";


fn calculate_prio(character: char) -> u32 {
    if (character >= 'A') && (character <= 'Z') {
        character as u32 - 65 + 27  // gets it to above 27
    } else if (character >= 'a') && (character <= 'z') {
        character as u32 - 97 + 1 // gets it to above 1
    } else {
        panic!("Unexpected character '{}'", character);
    }
}

fn main() {
    let file: String = fs::read_to_string(PATH_TO_FILE).expect("File not found");
    let mut sum: u32 = 0;

    for line in file.lines() {
        // get byte arrays of each half line turn it into a vector and dedupe them
        let s1 = line.split_at(line.len()/2).0.chars().collect_vec().into_iter().unique();
        let s2 = line.split_at(line.len()/2).1.chars().collect_vec().into_iter().unique();

        for c1 in s1.clone() {
            for c2 in s2.clone() {
                if c1 == c2 {
                sum += calculate_prio(c2);
                }
            }
        }
    }

    println!("{}", sum);
}
