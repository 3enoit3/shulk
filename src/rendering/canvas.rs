
use std::iter;
use std::iter::FromIterator;

pub struct Canvas {
    w: u32,
    h: u32,
    chars: Vec<char>,
}

impl Canvas {
    pub fn new(w: u32, h: u32) -> Canvas {
        let chars: Vec<char> = iter::repeat(' ').take((w * h) as usize).collect();
        Canvas{w, h, chars}
    }

    pub fn draw_char(&mut self, x: u32, y: u32, c: char) {
        if x >= self.w || y >= self.h {
            return;
        }
        let offset = (y * self.w + x) as usize;
        self.chars[offset] = c;
    }

    pub fn draw_string(&mut self, x: u32, y: u32, s: &str) {
        if x >= self.w || y >= self.h {
            return;
        }
        let mut i = x;
        for c in s.chars() {
            self.draw_char(i, y, c);

            i += 1;
            if i >= self.w {
                return;
            }
        }
    }

    pub fn draw_box(&mut self, x: u32, y: u32, w: u32, h: u32) {
        self.draw_char(x, y, '┌');
        self.draw_char(x+w, y, '┐');
        self.draw_char(x, y+h-1, '└');
        self.draw_char(x+w, y+h-1, '┘');

        for i in x+1..x+w {
            self.draw_char(i, y, '─');
            self.draw_char(i, y+h-1, '─');
        }
        for i in y+1..(y+h-1) {
            self.draw_char(x, i, '│');
            self.draw_char(x+w, i, '│');
        }
    }

    pub fn draw_connector(&mut self, x0: u32, y0: u32, x1: u32, y1: u32) {
        for x in x0+1..x1 {
            self.draw_char(x, y0, '─');
        }
        if y0 == y1 {
            return;
        }
        if y0 > y1 {
            self.draw_char(x1, y0, '┘');
            for y in y1+1..y0 {
                self.draw_char(x1, y, '│');
            }
        }
        else {
            self.draw_char(x1, y0, '┐');
            for y in y0+1..y1 {
                self.draw_char(x1, y, '│');
            }
        }
    }

    pub fn to_string(&self) -> String {
        let len = self.chars.len() + self.h as usize;
        let mut chars = Vec::<char>::with_capacity(len);
        for y in 0..self.h {
            let from = (y * self.w) as usize;
            let to = from + self.w as usize;
            chars.extend_from_slice(&self.chars[from..to]);
            chars.push('\n');
        }
        String::from_iter(chars)
    }
}
