#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn load(&mut self, contents: &str) {
        let lines: Vec<_> = contents.split("\n").map(|s| String::from(s)).collect();
        self.lines = lines;
    }
}
