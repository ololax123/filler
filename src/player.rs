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
                    self.min_x = i;
                    self.min_y = y;
                    self.max_x = i;
                    self.max_y = y;
                    found_player_start = true;
                    break; // We found a start position, no need to continue
                }
            }

            for (i, c) in line.chars().enumerate() {
                if opponent.start_chars.contains(c) {
                    opponent.start_x = i;
                    opponent.start_y = y;
                    opponent.min_x = i;
                    opponent.min_y = y;
                    opponent.max_x = i;
                    opponent.max_y = y;
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

    pub fn priority_direction(&mut self, opponent: Player) -> (bool, bool) {
        let mut to_return: (bool, bool) = (false, false);
        if self.start_x < opponent.start_x {
            to_return.0 = true;
        } else if self.start_x > opponent.start_x {
            to_return.0 = false;
        } else if self.start_y < opponent.start_y {
            to_return.1 = true;
        } else if self.start_y > opponent.start_y {
            to_return.1 = false;
        }
        to_return
    }
}
