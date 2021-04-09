
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
        Canvas{w:w, h:h, chars:chars}
    }

    pub fn drawChar(&mut self, x: u32, y: u32, c: char) {
        if x >= self.w || y >= self.h {
            return;
        }
        let offset = (y * self.w + x) as usize;
        self.chars[offset] = c;
    }

    pub fn drawString(&mut self, x: u32, y: u32, s: &str) {
        if x >= self.w || y >= self.h {
            return;
        }
        let mut i = x;
        for c in s.chars() {
            self.drawChar(i, y, c);

            i += 1;
            if i >= self.w {
                return;
            }
        }
    }

    pub fn toString(&self) -> String {
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
    let mut canvas = Canvas::new(w, h + 4);

    // Visuals
    for v in visuals.iter() {
        canvas.drawChar(v.x, v.y, render_img(v.content));
    }

    // Texts
    let mut y = h + 2;
    for t in texts.iter() {
        match t {
            Text::ItemAnnotation(id, s) => canvas.drawString(0, y, s),
            _ => (),
        };
        y += 1;
    }

    canvas.toString()
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
