use std::env;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    env::set_var("RUST_BACKTRACE", "1");

    // Read the exec line and determine the player
    let exec_line = lines
        .next()
        .expect("Expected exec line")
        .expect("Failed to read line");
    let player = if exec_line.contains("p1") { "p1" } else { "p2" };
    let opponent = if player.contains("1") { "p2" } else { "p1" };
    println!("Player: {}", player);

    // Read the Anfield size line
    let anfield_line = lines
        .next()
        .expect("Expected Anfield size line")
        .expect("Failed to read line");
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
    println!(
        "Anfield Width: {}, Height: {}",
        anfield_width, anfield_height
    );

    // Skip the colon line
    lines
        .next()
        .expect("Expected colon line")
        .expect("Failed to read line");

    // Read the Anfield grid and find the starting position
    let mut my_start_position = (0, 0);
    let my_start_char = if player == "p1" { '@' } else { '$' };
    let mut found_my_start = false;

    let mut opponent_start_position = (0, 0);
    let opponent_start_char = if player == "p1" { '$' } else { '@' };
    let mut found_opponent_start = false;
    for y in 0..anfield_height {
        let line = lines
            .next()
            .expect("Expected Anfield grid line")
            .expect("Failed to read line");
        if let Some(x) = line.find(my_start_char) {
            my_start_position = (x, y);
            found_my_start = true;
            //break; // We found the start position, no need to continue
        }
        if let Some(x) = line.find(opponent_start_char) {
            opponent_start_position = (x, y);
            found_opponent_start = true;
        }
    }

    if !found_my_start {
        eprintln!("Failed to find start position for player {}", player);
        return;
    }
    if !found_opponent_start {
        eprintln!("Failed to find start position for player {}", opponent);
    }

    println!(
        "My start Position: ({}, {})",
        my_start_position.0, my_start_position.1
    );
    println!(
        "Opponent startposition({}, {})",
        opponent_start_position.0, opponent_start_position.1
    );

    // Read the Piece size line
    let piece_line = lines
        .next()
        .expect("Expected piece size line")
        .expect("Failed to read line");
    println!("The piece size is {} ", piece_line);
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

    let (piece_width, piece_height) = (piece_sizes[0], piece_sizes[1]);
    println!("Piece Width: {}, Height: {}", piece_width, piece_height);

    // Skip the colon line
    // lines
    //     .next()
    //     .expect("Expected colon line")
    //     .expect("Failed to read line");

    // Read the piece grid
    let mut piece = Vec::new();
    for _ in 0..piece_height {
        let line = lines
            .next()
            .expect("Expected piece grid line")
            .expect("Failed to read line");
        piece.push(line.chars().collect::<Vec<char>>());
    }

    // Print the piece just for confirmation
    println!("Piece:");
    for row in piece.iter() {
        println!("{:?}", row);
    }
}
