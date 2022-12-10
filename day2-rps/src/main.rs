use std::fs;

/**
 * The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors)
 *  plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
 */
#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum RpsMove   {
    Nop = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

fn main() {
    let file_path = "src/input.txt";
    // let file_path = "src/test.txt";
    let file = fs::read_to_string(file_path).expect("ERROR reading in file");
    
    //split the string on newlines, now each iterator should contain a opponent move and our move
    let moves = file
        .split("\n")
        .map(|f| score_round(convert_move_to_shape(f.trim())) )
        .collect::<Vec<i32>>();

    let sum:i32 = moves.iter().sum();
    println!("first answer: {}", sum);

    let moves2 = file
    .split("\n")
    .map(|f| score_round(decide_move(f.trim())) )
    .collect::<Vec<i32>>();

    let sum2:i32 = moves2.iter().sum();
    println!("second answer: {}", sum2);

}
fn convert_move_to_shape(s: &str) -> (RpsMove,RpsMove) {
    if s.is_empty() { return (RpsMove::Nop, RpsMove::Nop); }
    let mut first:RpsMove = RpsMove::Nop;
    let mut last: RpsMove = RpsMove::Nop;
    // convert chars 
    for (i, c) in s.to_ascii_lowercase().chars().enumerate() {
        if i == 0 {
            match c {
                'a' => first = RpsMove::Rock,
                'b' => first = RpsMove::Paper,
                'c' => first = RpsMove::Scissors,
                _   => panic!("Unexpected character"),
            }

        }else if i == 2 {
            match c {
                'x' => last = RpsMove::Rock,
                'y' => last = RpsMove::Paper,
                'z' => last = RpsMove::Scissors,
                _   => panic!("Unexpected character"),
            }
        }
    }
    return (first, last);
}

fn score_round((first, last): (RpsMove, RpsMove)) -> i32 {
    if first == RpsMove::Nop || last == RpsMove::Nop { return 0; }
    // this is disgusting, implement a circular queue instead
    // (0 if you lost, 3 if the round was a draw, and 6 if you won)
    if first == last  {
        //tie
        return last as i32 + 3;
    } else if first == RpsMove::Rock && last == RpsMove::Scissors {
        // wrap around
        return last as i32 + 0;
    } else if first == RpsMove::Scissors && last == RpsMove::Rock  {
        // wrap around
        return last as i32 + 6
    } else if first > last {
        // loss
        return last as i32 + 0;
    } else if first < last {
        // win
        return last as i32 + 6;
    }
    
    return  0;
}

/**
 * Decides what move to make
 */
fn decide_move(s: &str) -> (RpsMove,RpsMove) {
    if s.is_empty() || s.len() < 3 { return (RpsMove::Nop, RpsMove::Nop); }
    let mut first:RpsMove = RpsMove::Nop;
    let mut last: RpsMove = RpsMove::Nop;
    let mut index:i32 = 0;
    let lose:i32 = 1;
    let draw:i32 = 2;
    // convert chars 
    for (i, c) in s.to_ascii_lowercase().chars().enumerate() {
        if i == 0 {
            match c {
                'a' => first = RpsMove::Rock,
                'b' => first = RpsMove::Paper,
                'c' => first = RpsMove::Scissors,
                _   => panic!("Unexpected character"),
            }

        }else if i == 2 {
            match c {
                // lose
                'x' => index = ((first as i32 + lose).abs() % 3 ) + 1,
                // draw
                'y' => index = ((first as i32 + draw).abs() % 3 ) + 1,
                // win
                'z' => index = ((first as i32 ).abs() % 3 ) + 1,
                _   => panic!("Unexpected character"),
            }
        }
    }
    last = match index {
        1 => RpsMove::Rock,
        2 => RpsMove::Paper,
        3 => RpsMove::Scissors,
        _ => panic!("Unexpected index {}", format!("index = {} and first = {:?} and s = {}", index, first, s)),
    };

    return (first, last);
}