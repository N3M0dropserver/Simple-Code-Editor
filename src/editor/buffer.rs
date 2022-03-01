extern crate ropey;
use ropey::Rope;

pub struct Buffer {
    content: Rope,
}

impl Buffer {
    pub fn new(file: &str) -> Buffer {
        let content = Rope::from_reader(std::fs::File::open(file).unwrap()).unwrap();
        Buffer { content: content }
    }

    pub fn get_line_at(&self, line: usize) -> String {
        String::from(self.content.line(line))
    }

    pub fn get_lines_count(&self) -> usize {
        self.content.len_lines()
    }

    pub fn get_lines(&self, from: usize, to: usize) -> String {
        let max_len = self.content.len_lines();
        let start = self.content.line_to_char(from);
        let end = if max_len > to {
            self.content.line_to_char(to)
        } else {
            self.content.line_to_char(max_len)
        };
        String::from(self.content.slice(start..end))
    }

    pub fn find_pos(&self, row: usize, col: usize) -> usize {
        let mut pos: usize = 0;

        for (rowpos, line) in self.content.lines().enumerate() {
            if rowpos == row {
                return pos + col;
            }
            pos += line.len_chars();
        }
        return pos + col;
    }

    pub fn insert_char(&mut self, row: usize, col: usize, c: char) {
        self.content.insert_char(self.find_pos(row, col), c);
    }

    pub fn remove_char(&mut self, row: usize, col: usize) {
        let pos = self.find_pos(row, col);
        if pos == 0 {
            return;
        }
        self.content.remove(pos - 1..pos);
    }
}
