
use std::iter;
use std::iter::FromIterator;
use std::cmp;

pub struct Visual {
    pub content: u32,
    pub x: u32,
    pub y: u32,
}

pub fn render(visuals: &[Visual]) -> String {
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
    for v in visuals.iter() {
        let i: u32 = v.y * row_len + v.x;
        chars[i as usize] = render_img(v.content);
    }
    for y in 0..h {
        let i: u32 = y * row_len + w;
        chars[i as usize] = '\n';
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
