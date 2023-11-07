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
}
