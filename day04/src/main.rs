use std::{fs, ops::Range};

const PATH_TO_FILE: &str = "input";

fn to_range(range: String) -> Range<i32> {
    let min = range.split_at(range.find("-").unwrap()).0;
    let max = range.split_at(range.find("-").unwrap() + 1).1;
    Range { start: i32::from_str_radix(min, 10).unwrap(), end: i32::from_str_radix(max, 10).unwrap()}
}

fn read_line(line: String) -> (Range<i32>, Range<i32>){
    let comma_pos = line.find(',').unwrap(); // split at ','
    let iterator = line.chars();
    // transform string with form '1-4' to range with start 1 and end 4
    let r1 = to_range(iterator.clone().take(comma_pos).collect());
    let r2 = to_range(iterator.skip(comma_pos+1).take(line.len()-comma_pos).collect());
    (r1, r2)
}


fn is_containted(r1: Range<i32>, r2: Range<i32>) -> bool {
    // check if one range is contained in another

    // range 1 contains range 2 or vice versa
    (r1.start <= r2.start) && (r1.end >= r2.end) ||
    (r2.start <= r1.start) && (r2.end >= r1.end)
}

fn does_overlap(r1: Range<i32>, r2: Range<i32>) -> bool {
    // range 1 and range 2 overlap at all
    !(r1.start > r2.end || r2.start > r1.end)
}


fn main() {
    let file: String = fs::read_to_string(PATH_TO_FILE).expect("File not found");
    let mut range_vec: Vec<(Range<i32>, Range<i32>)> = Vec::new(); 
    for line in file.lines() {
        let (r1, r2) = read_line(line.to_string());

        range_vec.push((r1, r2));
    }

    let part1 = range_vec.iter().map(|(r1, r2)| is_containted(r1.clone(), r2.clone())).filter(|&x| x == true).count();
    let part2 = range_vec.iter().map(|(r1, r2)| does_overlap(r1.clone(), r2.clone())).filter(|&x| x == true).count();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
