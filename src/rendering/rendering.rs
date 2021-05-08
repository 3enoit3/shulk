
use std::cmp;

use crate::graphics;
use super::canvas;
use super::layout;

pub fn render(visuals: &[graphics::Visual], texts: &[graphics::Text]) -> String {
    let (w, h) = layout::get_span(visuals);
    let text_len = layout::get_max_text_len(texts);
    let vx = text_len + 10;
    let vy = 5;
    let mut canvas = canvas::Canvas::new(vx + w, vy + h);


    // Visuals
    for v in visuals.iter() {
        canvas.draw_char(vx + v.x, vy + v.y, render_img(v.content));
    }

    // Texts
    let mut annotations = layout::collect(visuals, texts);
    layout::place(&mut annotations);
    for a in annotations.iter() {
        canvas.draw_string(a.pos.x+1, a.pos.y+1, &a.text);
        canvas.draw_box(a.pos.x, a.pos.y, a.w, a.h);
        canvas.draw_connector(a.w, a.pos.y+1, vx+a.target_pos.x, vy+a.target_pos.y);
    }

    canvas.to_string()
}

fn render_img(img: u32) -> char {
    let imgs = vec![' ', '□', '△', '▽', '>', '<'];
    match imgs.get(img as usize) {
        Some(c) => *c,
        None => 'X',
    }
}
