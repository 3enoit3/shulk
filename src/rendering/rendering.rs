
use std::cmp;

use crate::graphics;
use super::canvas;
use super::layout;

pub fn render(visuals: &[graphics::Visual], texts: &[graphics::Text]) -> String {
    let imgs = vec![' ', '□', '△', '▽', '>', '<'];
    let render_img = |img| {
        match imgs.get(img as usize) {
            Some(c) => *c,
            None => 'X',
        }
    };

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
    let mut y = 5;
    for t in texts.iter() {
        match t {
            graphics::Text::ItemAnnotation(id, s) => {
                canvas.draw_string(1, y+1, s);
                canvas.draw_box(0, y, (s.len() + 1) as u32, 3);
            },
            _ => (),
        };
        y += 4;
    }

    canvas.to_string()
}
