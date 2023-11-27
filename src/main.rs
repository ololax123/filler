use r2_d2::anfield::*;
use r2_d2::piece::*;
use r2_d2::player::*;
// use std::fs::OpenOptions;
use r2_d2::fransfan::*;
use std::io::{self, BufRead};

fn main() {
    // let path = "output.txt";
    // let mut file = OpenOptions::new()
    //     .append(true) // Set the append mode
    //     .create(true) // Create the file if it doesn't exist
    //     .open(path)
    //     .expect("Failed to open file"); // Handle the Result

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut player = Player::new();
    let mut opponent = Player::new();

    let line = lines
        .next()
        .expect("Expected exec line")
        .expect("Failed to read line");
    if line.contains("exec") {
        player.get_start_info(&line);
        (opponent.num, opponent.start_chars) = if player.num == 1 {
            (2, "s$".to_string())
        } else {
            (1, "a@".to_string())
        };
    }

    loop {
        let mut anfield = Anfield::new();
        let mut anfield_area = String::new();
        let mut piece = Piece::new();
        //Check Anfield
        let line = lines
            .next()
            .expect("Expected next line")
            .expect("Failed to read line");
        anfield.get_size(&line);
        lines.next();
        //Loop throught the anfield and save it.
        for _ in 0..anfield.height {
            let line = lines
                .next()
                .expect("Expected Anfield grid line")
                .expect("Failed to read line");
            anfield_area.push_str(&line);
            anfield_area.push('\n'); // Adds a newline character.
        }

        //Get player start positions
        player.get_start_positions(&anfield_area, &mut opponent);

        //Skip down to piece
        let line = lines
            .next()
            .expect("Expected next line")
            .expect("Failed to read line");

        piece.get_size(&line);
        //Save piece grid
        for _ in 0..piece.height {
            let line = lines
                .next()
                .expect("Expected piece grid line")
                .expect("Failed to read line");
            piece.grid.push(line.chars().collect::<Vec<char>>());
        }

        //Calculate profits!
        // let (go_x, go_y) = player.priority_direction(&opponent);
        // let (place_x, place_y) = piece.place(&mut player, go_x, go_y, anfield_area);
        let white_flag_land: Vec<Vec<char>> = anfield_area
            .lines()
            .map(|line| line.chars().skip(4).collect())
            .collect();

        let (place_x, place_y) = place_piece(
            &white_flag_land,
            &piece.grid,
            &(player.start_chars.chars().collect()),
            &(opponent.start_chars.chars().collect()),
        );
        print!("{} {}\n", place_x, place_y);
    }
}
