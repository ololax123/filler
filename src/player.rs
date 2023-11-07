#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub num: usize,
    pub start_chars: String,
    pub start_x: usize,
    pub start_y: usize,
    pub max_x: usize,
    pub max_y: usize,
    pub min_x: usize,
    pub min_y: usize,
}

impl Player {
    pub fn new() -> Self {
        Player {
            num: 0,
            start_chars: "".to_string(),
            start_x: 0,
            start_y: 0,
            max_x: 0,
            max_y: 0,
            min_x: 0,
            min_y: 0,
        }
    }
    pub fn get_start_info(&mut self, input: &str) {
        self.num = if input.contains("p1") { 1 } else { 2 };
        self.start_chars = if self.num == 1 {
            "a@".to_string()
        } else {
            "s$".to_string()
        };
    }

    pub fn get_start_positions(
        &mut self,
        lines: String,
        player_start_chars: String,
        opponent: &mut Player,
    ) {
        let mut found_player_start = false;
        let mut found_opponent_start = false;

        for (y, line) in lines.lines().enumerate() {
            for (i, c) in line.chars().enumerate() {
                if player_start_chars.contains(c) {
                    self.start_x = i;
                    self.start_y = y;
                    found_player_start = true;
                    break; // We found a start position, no need to continue
                }
            }

            for (i, c) in line.chars().enumerate() {
                if opponent.start_chars.contains(c) {
                    opponent.start_x = i;
                    opponent.start_y = y;
                    found_opponent_start = true;
                    break; // We found a start position, no need to continue
                }
            }
        }

        if !found_player_start {
            eprintln!(
                "Failed to find start position for player {}",
                player_start_chars
            );
        }
        if !found_opponent_start {
            eprintln!(
                "Failed to find start position for player {}",
                opponent.start_chars
            );
        }
    }
}
