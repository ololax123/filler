#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub num: usize,
    pub start_chars: Vec<char>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            num: 0,
            start_chars: Vec::new(),
        }
    }
    pub fn get_start_info(&mut self, input: &str) {
        self.num = if input.contains("p1") { 1 } else { 2 };
        let chars_str = if self.num == 1 { "a@" } else { "s$" };
        self.start_chars = chars_str.chars().collect();
    }
}
