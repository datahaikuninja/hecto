#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn load(&mut self, contents: &str) {
        let lines: Vec<_> = contents.lines().map(String::from).collect();
        self.lines = lines;
    }
}
