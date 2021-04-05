
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

pub fn render(visuals: &[Visual], texts: &[Text]) -> String {
    let imgs = vec![' ', '□', '△', '▽', '>', '<'];
    let render_img = |img| {
        match imgs.get(img as usize) {
            Some(c) => *c,
            None => 'X',
        }
    };

    let (w, h) = get_span(visuals);
    let row_len = w + 1;
    let mut chars: Vec<char> = iter::repeat(' ').take((row_len * h) as usize).collect();
    let mut paint = |x, y, c| {
        let i: u32 = y * row_len + x;
        chars[i as usize] = c;
    };

    for y in 0..h {
        paint(w, y, '\n');
    }

    for v in visuals.iter() {
        paint(v.x, v.y, render_img(v.content));
    }

    String::from_iter(chars)
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
