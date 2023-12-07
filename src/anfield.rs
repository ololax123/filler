pub struct Anfield {
    pub height: usize,
    pub width: usize,
}

impl Default for Anfield {
    fn default() -> Self {
        Self::new()
    }
}
impl Anfield {
    pub fn new() -> Self {
        Anfield {
            height: 0,
            width: 0,
        }
    }
    pub fn get_size(&mut self, input: &str) {
        let sizes: Vec<usize> = input
            .split_whitespace()
            .skip(1) // Skip the name 'Anfield'
            .take(2) // Take the next two numbers
            .map(|num| {
                num.trim_end_matches(':')
                    .parse()
                    .expect("Failed to parse Anfield size")
            })
            .collect();
        self.width = sizes[0];
        self.height = sizes[1];
    }
}
