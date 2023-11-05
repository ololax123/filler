use std::fs;
use std::io::{self, BufRead};

fn main() {
    let path = "output.txt";

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut contains_piece = false;
    let mut input = String::new();
    let mut piece_width = 0;
    let mut piece_height = 0;

    while contains_piece == false {
        let line = lines
            .next()
            .expect("Expected Anfield grid line")
            .expect("Failed to read line");
        input = input + &line + "\n";
        if line.contains("Piece") {
            contains_piece = true;
            let piece_sizes: Vec<usize> = line
                .split_whitespace()
                .skip(1) // Skip the word 'Piece'
                .take(2) // Take the next two numbers
                .map(|num| {
                    num.trim_end_matches(':')
                        .parse()
                        .expect("Failed to parse piece size")
                })
                .collect();

            (piece_width, piece_height) = (piece_sizes[0], piece_sizes[1]);
        }
    }
    for _ in 0..piece_height {
        let line: String = lines
            .next()
            .expect("Expected piece grid line")
            .expect("Failed to read line");
        input = input + &line + "\n";
    }
    let input_clone = input.clone();
    let _ = fs::write(path, input_clone);
    let mut input = input.lines();
    // Read the exec line and determine the player
    let exec_line = input.next().expect("Expected exec line");
    let player = if exec_line.contains("p1") { "p1" } else { "p2" };
    let opponent = if player.contains("1") { "p2" } else { "p1" };

    // Read the Anfield size line
    let anfield_line = input.next().expect("Expected Anfield size line");
    let sizes: Vec<usize> = anfield_line
        .split_whitespace()
        .skip(1) // Skip the name 'Anfield'
        .take(2) // Take the next two numbers
        .map(|num| {
            num.trim_end_matches(':')
                .parse()
                .expect("Failed to parse Anfield size")
        })
        .collect();

    let (anfield_width, anfield_height) = (sizes[0], sizes[1]);

    // Skip the colon line
    input.next().expect("Expected colon line");

    // Read the Anfield grid and find the starting position
    let mut my_start_position = (0, 0);
    let my_start_chars = if player == "p1" { "a@" } else { "s$" };
    let mut found_my_start = false;

    let mut opponent_start_position = (0, 0);
    let opponent_start_chars = if player == "p1" { "s$" } else { "a@" };
    let mut found_opponent_start = false;
    for y in 0..anfield_height {
        let line = input.next().expect("Expected Anfield grid line");
        for (i, c) in line.chars().enumerate() {
            if my_start_chars.contains(c) {
                my_start_position = (i, y);
                found_my_start = true;
                break; // We found a start position, no need to continue
            }
        }

        for (i, c) in line.chars().enumerate() {
            if opponent_start_chars.contains(c) {
                opponent_start_position = (i, y);
                found_opponent_start = true;
                break; // We found a start position, no need to continue
            }
        }
    }

    if !found_my_start {
        eprintln!("Failed to find start position for player {}", player);
        return;
    }
    if !found_opponent_start {
        eprintln!("Failed to find start position for player {}", opponent);
    }

    // Read the Piece size line
    let piece_line = input.next().expect("Expected piece size line");
    let piece_sizes: Vec<usize> = piece_line
        .split_whitespace()
        .skip(1) // Skip the word 'Piece'
        .take(2) // Take the next two numbers
        .map(|num| {
            num.trim_end_matches(':')
                .parse()
                .expect("Failed to parse piece size")
        })
        .collect();

    (piece_width, piece_height) = (piece_sizes[0], piece_sizes[1]);

    // Skip the colon line
    // lines
    //     .next()
    //     .expect("Expected colon line")
    //     .expect("Failed to read line");

    // Read the piece grid
    let mut piece = Vec::new();
    for _ in 0..piece_height {
        let line: &str = input.next().expect("Expected piece grid line");
        piece.push(line.chars().collect::<Vec<char>>());
    }

    // Print the piece just for confirmation
    for row in piece.iter() {
        println!("{:?}", row);
    }
}
