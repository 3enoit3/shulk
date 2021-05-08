
use std::cmp;
use log::info;

use crate::graphics;
use super::geometry;

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

pub struct Annotation {
    pub id: u32,
    pub text: String,
    pub w: u32,
    pub h: u32,
    pub target_pos: geometry::Point,
    pub pos: geometry::Point,
}

pub fn collect(visuals: &[graphics::Visual], texts: &[graphics::Text]) -> Vec<Annotation> {
    let visual_ids: Vec<&graphics::Visual> = visuals.iter().filter(|v| match v.id {
        Some(id) => true,
        _ => false,
    }).collect();

    let mut annotations = Vec::<Annotation>::new();
    for v in visual_ids.iter() {
        match v.id {
            Some(v_id) => {
                info!("test");
                for t in texts.iter() {
                    match t {
                        graphics::Text::ItemAnnotation(a_id, s) => {
                            if v_id == *a_id {
                                let v_pos = geometry::Point{x:v.x, y:v.y};
                                let annotation = Annotation{id: v_id, text:s.clone(), w:s.len() as u32 + 2, h:3, target_pos:v_pos, pos: geometry::Point{x:0, y:0}};
                                annotations.push(annotation);
                            }
                        },
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    annotations
}

pub fn place(annotations: &mut [Annotation]) {
    if annotations.len() == 0 {
        return
    }

    annotations.sort_by_key(|a| a.target_pos.y * 1000 + a.target_pos.x);

    let mut i = 0;
    for a in annotations.iter_mut() {
        a.pos = geometry::Point{x:0, y:i};
        i += 3
    }

    // build layout
    let mut layout: Vec<LayoutAnnotation> = vec![];
    for a in annotations.iter() {
        layout.push(LayoutAnnotation{id:a.id, y:a.pos.y, target:a.target_pos});
    }

    // optimize
    loop {
        let mut moves = build_all_moves(&layout);
        moves.sort_by_key(|a| evaluate_move(a));
        let best = moves[0].clone();
        if evaluate_move(&best) >= evaluate_move(&layout) {
            break;
        }
        layout = best;
    }

    // apply layout
    for l in layout.iter() {
        for a in annotations.iter_mut() {
            if a.id == l.id {
                a.pos.y = l.y;
            }
        }
    }
}

#[derive(Clone)]
pub struct LayoutAnnotation {
    pub id: u32,
    pub y: u32,
    pub target: geometry::Point,
}

impl LayoutAnnotation {
    fn get_anchor(&self) -> geometry::Point {
        geometry::Point{x:0, y:self.y + 1}
    }

    fn get_horizontal(&self) -> geometry::Segment {
        let anchor = self.get_anchor();
        let target = geometry::Point{x:self.target.x, y:anchor.y};
        geometry::Segment{p1:anchor, p2:target}
    }

    fn get_vertical(&self) -> geometry::Segment {
        let anchor = geometry::Point{x:self.target.x, y:self.get_anchor().y};
        if self.target.y > anchor.y {
            return geometry::Segment{p1:anchor, p2:self.target.clone()};
        }
        else {
            return geometry::Segment{p1:self.target.clone(), p2:anchor};
        }
    }

    pub fn get_overlap_score(a1: &LayoutAnnotation, a2: &LayoutAnnotation) -> u32 {
        let common = geometry::Segment::get_common(&a1.get_vertical(), &a2.get_vertical());
        if common > 0 {
            return common;
        }
        if geometry::Segment::are_crossing(&a1.get_horizontal(), &a2.get_vertical()) || geometry::Segment::are_crossing(&a2.get_horizontal(), &a1.get_vertical()) {
            return 100;
        }
        return 0;
    }
}

fn build_all_moves(init: &[LayoutAnnotation]) -> Vec<Vec<LayoutAnnotation>> {
    let mut res: Vec<Vec<LayoutAnnotation>> = vec![];
    for i in 0..init.len()-1  {
        res.push(swap_two(init, i));
    }
    res
}

fn swap_two(init: &[LayoutAnnotation], first: usize) -> Vec<LayoutAnnotation> {
    let mut res = Vec::<LayoutAnnotation>::new();
    for i in 0..first {
        res.push(init[i].clone());
    }

    res.push(init[first+1].clone());
    res[first].y = init[first].y;
    res.push(init[first].clone());
    res[first+1].y = init[first+1].y;

    for i in first+2..init.len() {
        res.push(init[i].clone());
    }
    res
}

fn evaluate_move(annotations: &[LayoutAnnotation]) -> u32 {
    let mut score: u32 = 0;
    for i in 0..annotations.len()-1 {
        for j in i+1..annotations.len() {
            score += LayoutAnnotation::get_overlap_score(&annotations[i], &annotations[j]);
        }
    }
    score
}
