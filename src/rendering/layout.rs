
use std::cmp;

use crate::graphics;

pub fn get_span(visuals: &[graphics::Visual]) -> (u32, u32) {
    let mut w: u32 = 0;
    let mut h: u32 = 0;
    for v in visuals.iter() {
        w = cmp::max(w, v.x + 1);
        h = cmp::max(h, v.y + 1);
    }
    (w, h)
}

pub fn get_max_text_len(texts: &[graphics::Text]) -> u32 {
    let mut m: u32 = 0;
    for t in texts.iter() {
        match t {
            graphics::Text::ItemAnnotation(_, s) => m = cmp::max(m, s.len() as u32),
            _ => (),
        }
    }
    m
}
