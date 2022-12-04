use std::collections::HashMap;
use std::fs;

// A/X = Rock     = 1 P
// B/Y = Paper    = 2 P
// C/Z = Scissors = 3 P
//
// lost = 0
// draw = 3
// won  = 6

// File path since i'm lazy...again
const PATH_TO_FILE: &str = "input";

fn assert_win(opponent_hand: i32, my_hand: i32) -> i32{
    if opponent_hand.eq(&my_hand){
        // draw
        3 + my_hand
    } else if (opponent_hand == (1) && my_hand == (2)) || // rock, paper
              (opponent_hand == (2) && my_hand == (3)) || // paper, scissors
              (opponent_hand == (3) && my_hand == (1)) {  // scissors, rock
        // win
        6 + my_hand
    } else {
        // lose
        0 + my_hand
    }
}

fn main() {
    let mut score = HashMap::new();

    score.insert("A", 1);
    score.insert("B", 2);
    score.insert("C", 3);
    score.insert("X", 1);
    score.insert("Y", 2);
    score.insert("Z", 3);
    
    // Read file
    let file_contents= fs::read_to_string(PATH_TO_FILE).expect("Could not read file");

    let mut scores: Vec<i32> = Vec::new();
    
    for line in file_contents.lines() {
        let mut hands = line.split_whitespace();
        let opponent = score.get(hands.next().unwrap()).unwrap();
        let my       = score.get(hands.next().unwrap()).unwrap();
        scores.push(assert_win(*opponent, *my));
    }

    let total: i32= scores.iter().sum();
    println!("Total score: {}", total);

}
