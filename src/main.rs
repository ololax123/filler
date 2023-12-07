use r2_d2::anfield::*;
use r2_d2::piece::*;
use r2_d2::player::*;
use std::io::{self, BufRead};

fn main() {
    //Start listening for input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut player = Player::default();
    let mut opponent = Player::default();

    //First input should be which player we are, get it and save the info
    let line = lines
        .next()
        .expect("Expected exec line")
        .expect("Failed to read line");
    if line.contains("exec") {
        player.get_start_info(&line);
        (opponent.num, opponent.start_chars) = if player.num == 1 {
            (2, "s$".chars().collect())
        } else {
            (1, "a@".chars().collect())
        };
    }
    //Loop and wait for your turn
    loop {
        let mut anfield = Anfield::new();
        let mut anfield_area = <Vec<Vec<char>>>::new();
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
            anfield_area.push(line.chars().skip(4).collect::<Vec<char>>());
        }

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

        let (place_x, place_y) = place_piece(
            &anfield_area,
            &piece.grid,
            &(player.start_chars),
            &(opponent.start_chars),
        );
        println!("{} {}", place_x, place_y);
    }
}
