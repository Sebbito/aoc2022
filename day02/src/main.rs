use std::collections::HashMap;
use std::fs;

// File path since i'm lazy...again
const PATH_TO_FILE: &str = "input";

fn assert_win(opponent_hand: i32, my_hand: i32) -> i32{
    if opponent_hand.eq(&my_hand){
        // draw
        3 + my_hand
    } else if ((opponent_hand % 3) + 1).eq(&my_hand) {
        // win
        6 + my_hand
    } else {
        // lose
        0 + my_hand
    }
}

fn find_hand(opponent_hand: i32, result: i32) -> i32 {
    let mut my_hand: i32;

    if result == 3 { // draw
        my_hand = opponent_hand;
    } else if result == 6 { // win
        my_hand = (opponent_hand % 3) + 1;
    } else { // lose
        my_hand = opponent_hand + 2;
        if my_hand.gt(&3){
            my_hand %= 3;
        }
    }
    result + my_hand
}

fn main() {
    let mut score = HashMap::new();

    score.insert("A", 1); // Rock
    score.insert("B", 2); // Paper
    score.insert("C", 3); // Scissors
    score.insert("X", 1); // Rock
    score.insert("Y", 2); // Paper
    score.insert("Z", 3); // Scissors

    let mut win_value = HashMap::new();

    win_value.insert("X", 0); // lose
    win_value.insert("Y", 3); // draw
    win_value.insert("Z", 6); // win
    
    // Read file
    let file_contents= fs::read_to_string(PATH_TO_FILE).expect("Could not read file");

    let mut scores_p1: Vec<i32> = Vec::new();
    let mut scores_p2: Vec<i32> = Vec::new();
    
    // doing both parts in one loop i don't care
    for line in file_contents.lines() {
        let mut hands = line.split_whitespace();
        let opponent = score.get(hands.next().unwrap()).unwrap();

        let v2 = hands.next().unwrap();
        let my_hand = score.get(v2).unwrap(); // part 1
        let result =  win_value.get(v2).unwrap(); // part 2

        scores_p1.push(assert_win(*opponent, *my_hand));
        scores_p2.push(find_hand(*opponent, *result));
    }

    // part one
    let total: i32= scores_p1.iter().sum();
    println!("Part 1 Total score: {}", total);

    // part two
    let total: i32 = scores_p2.iter().sum();
    println!("Part 2 Total Score: {}", total);


}
