use crate::player::Player;

pub struct Piece {
    pub height: usize,
    pub width: usize,
    pub grid: Vec<Vec<char>>,
}

impl Piece {
    pub fn new() -> Self {
        Piece {
            height: 0,
            width: 0,
            grid: Vec::new(),
        }
    }
    pub fn minimize(&mut self) {
        //Strip all empty rows in the bottom and to the right
    }
    pub fn get_size(&mut self, input: &str) {
        let piece_sizes: Vec<usize> = input
            .split_whitespace()
            .skip(1) // Skip the word 'Piece'
            .take(2) // Take the next two numbers
            .map(|num| {
                num.trim_end_matches(':')
                    .parse()
                    .expect("Failed to parse piece size")
            })
            .collect();

        (self.width, self.height) = (piece_sizes[0], piece_sizes[1]);
    }
    pub fn place(
        &self,
        player: &mut Player,
        go_x: bool,
        go_y: bool,
        input: String,
    ) -> (usize, usize) {
        // Parse the playing field from the input string
        let field: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        // Find all possible placements where the piece overlaps with exactly one player's character
        let mut valid_placements = Vec::new();
        for y in 0..=field.len() {
            if y > player.max_y {
                continue;
            }
            for x in 4..=field[0].len() {
                if x > player.max_x {
                    continue;
                }
                if self.can_place_at(x, y, player, &field) {
                    valid_placements.push((x - 4, y));
                }
            }
        }
        // If there are no valid placements, we cannot place the piece
        if valid_placements.is_empty() {
            valid_placements.push((0, 0));
        }
        // Determine the optimal placement based on go_x and go_y
        let optimal_placement = self.find_optimal_placement(&valid_placements, go_x, go_y);

        optimal_placement
    }

    // Helper function to determine if the piece can be placed at a given position
    fn can_place_at(&self, x: usize, y: usize, player: &Player, field: &Vec<Vec<char>>) -> bool {
        let mut player_overlap = 0;

        // Iterate over each cell in the piece's grid
        for (dy, row) in self.grid.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                let field_x = x + dx;
                let field_y = y + dy;

                // Check if the piece is within the bounds of the field
                if field_y >= field.len() || field_x >= field[0].len() {
                    return false;
                }

                match (field[field_y][field_x], cell) {
                    // If the cell in the piece is part of the piece ('O') and the field is empty ('.'), it's a valid placement
                    ('.', 'O') => continue,

                    // If the cell in the piece is part of the piece ('O') and the field has the player's character,
                    // increment the overlap count
                    (player_char, 'O') if player.start_chars.contains(player_char) => {
                        player_overlap += 1;
                        if player_overlap > 1 {
                            return false;
                        }
                    }

                    // If the cell in the piece is part of the piece ('O') and the field has an opponent's piece ('#'),
                    // it's an invalid placement
                    ('#', 'O') => {
                        return false;
                    }

                    // If the cell in the piece is empty and the field has any character, it's also a valid placement
                    (_, '.') => continue,

                    // Any other case is invalid
                    _ => {
                        return false;
                    }
                }
            }
        }

        // The placement is only valid if there is exactly one overlap with the player's characters
        player_overlap == 1
    }
    // Helper function to find the optimal placement from a list of valid placements
    fn find_optimal_placement(
        &self,
        placements: &[(usize, usize)],
        go_x: bool,
        go_y: bool,
    ) -> (usize, usize) {
        placements
            .iter()
            .copied()
            .max_by(|&(x1, y1), &(x2, y2)| {
                match (go_x, go_y) {
                    (true, false) => x1.cmp(&x2), // Prioritize furthest right
                    (false, true) => y1.cmp(&y2), // Prioritize furthest bottom
                    (true, true) => {
                        // Prioritize bottom first, then right
                        match y1.cmp(&y2) {
                            std::cmp::Ordering::Equal => x1.cmp(&x2),
                            other => other,
                        }
                    }
                    _ => {
                        // If neither direction is set to true, or both are false, return the first (this shouldn't happen)
                        std::cmp::Ordering::Equal
                    }
                }
            })
            .expect("No valid placements found") // This assumes there is at least one valid placement
    }
}
