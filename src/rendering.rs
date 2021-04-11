
use std::iter;
use std::iter::FromIterator;
use std::cmp;

pub struct Visual {
    pub content: u32,
    pub x: u32,
    pub y: u32,
    pub id: Option<u32>,
}

pub enum Text {
    Title(String),
    ItemAnnotation(u32, String),
}

struct Canvas {
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

pub fn render(visuals: &[Visual], texts: &[Text]) -> String {
    let imgs = vec![' ', '□', '△', '▽', '>', '<'];
    let render_img = |img| {
        match imgs.get(img as usize) {
            Some(c) => *c,
            None => 'X',
        }
    };

    let (w, h) = get_span(visuals);
    let text_len = get_max_text_len(texts);
    let vx = text_len + 10;
    let vy = 5;
    let mut canvas = Canvas::new(vx + w, vy + h);

    // Visuals
    for v in visuals.iter() {
        canvas.draw_char(vx + v.x, vy + v.y, render_img(v.content));
    }

    // Texts
    let mut y = 5;
    for t in texts.iter() {
        match t {
            Text::ItemAnnotation(id, s) => {
                canvas.draw_string(1, y+1, s);
                canvas.draw_box(0, y, (s.len() + 1) as u32, 3);
            },
            _ => (),
        };
        y += 4;
    }

    canvas.to_string()
}

fn get_span(visuals: &[Visual]) -> (u32, u32) {
    let mut w: u32 = 0;
    let mut h: u32 = 0;
    for v in visuals.iter() {
        w = cmp::max(w, v.x + 1);
        h = cmp::max(h, v.y + 1);
    }
    (w, h)
}

fn get_max_text_len(texts: &[Text]) -> u32 {
    let mut m: u32 = 0;
    for t in texts.iter() {
        match t {
            Text::ItemAnnotation(_, s) => m = cmp::max(m, s.len() as u32),
            _ => (),
        }
    }
    m
}
