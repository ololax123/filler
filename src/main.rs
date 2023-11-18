use r2_d2::anfield::*;
use r2_d2::piece::*;
use r2_d2::player::*;
use std::io::{self, BufRead};
// use std::fs;

fn main() {
    // let path = "output.txt";

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut contains_piece = false;
    let mut input = String::new();
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

            piece_height = piece_sizes[1];
        }
    }

    for _ in 0..piece_height {
        let line: String = lines
            .next()
            .expect("Expected piece grid line")
            .expect("Failed to read line");
        input = input + &line + "\n";
    }
    let mut player = Player::new();
    let mut opponent = Player::new();
    let mut anfield = Anfield::new();
    let mut piece = Piece::new();

    // //Clone input and dump into file
    // let input_clone = input.clone();
    // let _ = fs::write(path, input_clone);

    // Start reading the input
    let mut input = input.lines();

    //Get the player & Opponent info
    let player_line = input.next().expect("Expected exec line");
    player.get_start_info(player_line);
    (opponent.num, opponent.start_chars) = if player.num == 1 {
        (2, "s$".to_string())
    } else {
        (1, "a@".to_string())
    };

    // Read the Anfield size line
    let anfield_line = input.next().expect("Expected Anfield size line");
    anfield.get_size(anfield_line);

    // Skip the numbered line
    input.next().expect("Expected numbered line");
    // Create a clone of the field grid
    let mut input_clone = String::new();
    for _ in 0..anfield.height {
        let line = input.next().expect("Expected Anfield grid line");
        input_clone.push_str(&line);
        input_clone.push('\n'); // Adds a newline character.
    }

    //Get the start positions of player and opponent
    player.get_start_positions(
        input_clone.clone(),
        player.start_chars.clone(),
        &mut opponent,
    );

    //Get the piece size
    let piece_line = input.next().expect("Expected piece size line");
    piece.get_size(piece_line);

    // Read the piece grid
    for _ in 0..piece_height {
        let line: &str = input.next().expect("Expected piece grid line");
        piece.grid.push(line.chars().collect::<Vec<char>>());
    }
    let (go_x, go_y) = player.priority_direction(opponent);
    let (place_x, place_y) = piece.place(&mut player, go_x, go_y, input_clone);
    println!("{} {}", place_x, place_y);
}
