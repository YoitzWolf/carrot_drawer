use std::collections::{HashMap, HashSet};
use std::f32::consts::PI;
use std::hash::Hash;
use std::ops::{Deref, RemAssign};
use glam::{Vec2, Vec3, Vec3Swizzles};
// use crate::core::vis_geometry::contour::PolygonList;
use crate::core::vis_geometry::Vertex;


#[derive(Clone, Debug)]
struct SegmentBinTree {
    pub id: usize,
    pub x_cord: f32,
    pub left:  Option<Box<SegmentBinTree>>,
    pub right: Option<Box<SegmentBinTree>>,
}

impl SegmentBinTree {
    pub fn new(id: usize, x_cord: f32) -> SegmentBinTree {
        Self {
            id, x_cord, left: None, right: None
        }
    }
    pub fn insert(&mut self, other: Self) -> anyhow::Result<()>{
        if self.id == other.id { return Err( anyhow::Error::msg("Self id is other id in bintree") ); }
        if self.x_cord > other.x_cord {
            // left
            match &mut self.left {
                None => {
                    self.left = Some(Box::new(other));
                    Ok(())
                }
                Some(x) => {
                    x.insert(other)
                }
            }
        } else {
            // right
            match &mut self.right {
                None => {
                    self.right = Some(Box::new(other));
                    Ok(())
                }
                Some(x) => {
                    x.insert(other)
                }
            }
        }
    }

    pub fn remove(&mut self, other_id: usize, other_cord: f32) -> anyhow::Result<()> {
        if self.id == other_id { return Err( anyhow::Error::msg("Self id is other id in bintree") ); }
        if self.x_cord > other_cord {
            // left
            match &mut self.left {
                None => {
                    Err(anyhow::Error::msg("Not found (remove left)"))
                }
                Some(x) => {
                    if (x.id == other_id) {
                        let x = self.left.take().unwrap();
                        if let Some(xl) = x.left { self.insert(*xl)?; }
                        if let Some(xl) = x.right { self.insert(*xl)?; }
                        self.left = None;
                        Ok(())
                    } else {
                        x.remove(other_id, other_cord)
                    }
                }
            }
        } else {
            // right
            match &mut self.right {
                None => {
                    Err(anyhow::Error::msg("Not found (remove right)"))
                }
                Some(x) => {
                    if (x.id == other_id) {
                        let x = self.right.take().unwrap();
                        if let Some(xl) = x.left { self.insert(*xl)?; }
                        if let Some(xl) = x.right { self.insert(*xl)?; }
                        self.right = None;
                        Ok(())
                    } else {
                        x.remove(other_id, other_cord)
                    }
                }
            }
        }
    }

    pub fn nearest_left(&mut self, x: f32, y: f32, points: &[Vec3], N: usize) -> anyhow::Result<usize> {
        let x_cord = {
            let point_st = points[self.id].xy();
            let point_end = points[(self.id + 1) % N].xy();
            let v = (point_st - point_end);
            let k = v.y / v.x;
            point_st.x + y/k
        };
        if x_cord < x { // TODO check is this correct or fix x_cord == x variant
            // self is at left to x
            // check if right is better
            match &mut self.right {
                None => {
                    // self is left to x
                    // self if last left-side
                    Ok(self.id)
                }
                Some(seg) => {
                    let res = seg.nearest_left(x, y, points, N);
                    if res.is_ok() {
                        // exist better
                        res
                    } else {
                        // yeah, self is best, all to right to self is really all to right to x
                        Ok(self.id)
                    }
                }
            }
        } else {
            // self is at right to x
            // checks left branch or call error
            match &mut self.left {
                None => {
                    // self is at right to x, it is a mistake!
                    // println!("{:?} >> {}, {}", self, x_cord, x);
                    Err(anyhow::Error::msg("Not found (nearest left searching)"))
                }
                Some(seg) => {
                    seg.nearest_left(x, y, points, N)
                }
            }
        }
    }
}


#[derive(Clone, Debug, Default)]
struct PolygonSelector {
    pub used: HashMap<(usize, usize), usize>,
}

impl PolygonSelector {
    pub fn new() -> PolygonSelector {
        PolygonSelector {
            used: Default::default(),
        }
    }

    pub fn select(
        &mut self,
        start: (usize, usize),
        points: &[Vec3],
        extra_segments: &HashMap<usize, Vec<usize>>,
    ) -> anyhow::Result< Vec<usize> > {
        let st_cnt = if let Some(st) = extra_segments.get(&start.0) {
            st.len()
        } else {
            0
        };
        if !self.used.contains_key(&start) {
            self.used.insert(start, 0);
        }
        let start_counter = self.used.get_mut(&start).unwrap();
        if *start_counter == st_cnt + 2 {
            Err(anyhow::Error::msg("No such new polygon"))
        } else {
            // try to find not overused path:
            let N = points.len();
            let mut cur = start.1;
            let mut vertexpath = vec![start.0];
            let mut vertex_set = HashSet::new();
            vertex_set.insert(start.0);
            let mut idx_next = (cur + 1) % N;
            let mut idx_prev = (cur + N - 1) % N;
            // let mut next_cur_vec;
            let mut last_edge = points[*(vertexpath.last().unwrap())] - points[cur];
            let mut check_edge = Vec3::new(0.0, 0.0, 0.0);
            let mut check= Vec3::new(0.0, 0.0, 0.0);
            let (_, is_right, next_cur_id) = {
                [
                    vec![ idx_next, idx_prev ],
                    if let Some(v) = extra_segments.get(&cur) { v.clone() } else { Vec::new() },
                ].concat().iter()
                    .filter(|&x| {
                            !vertex_set.contains(x) &&
                            match self.used.get(&(cur, *x)) {
                                None => { true }
                                Some(t) => {
                                    *t < 1 || *t < 2 && {
                                        match extra_segments.get(&cur) {
                                            None => { false }
                                            Some(pts) => {
                                                pts.contains(x) // cheks if &(cur, *x) is extra added
                                                // bcs, not-added edges are single-time-usable
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    ).map(
                        |this| {
                            check_edge = points[*this] - points[cur];
                            last_edge.z = 0.0;
                            check_edge.z = 0.0;
                            check = last_edge * check_edge;
                            (check.length() / (last_edge.length() * check_edge.length()), check.z > 0.0, *this)
                        }
                    ).fold(
                        (f32::INFINITY, false, usize::MAX),
                        |state, val| {
                            if state.0 > val.0 {
                                val.clone()
                            } else {
                                val
                            }
                        }
                    )
            };
            // println!("first segment detection. is right: {}; next cur: {}", is_right, next_cur_id);
            if next_cur_id == usize::MAX {
                return Err(anyhow::Error::msg("No such new polygon (first segment detection)"));
            }
            vertexpath.push(cur);
            vertex_set.insert(cur);
            cur = next_cur_id;
            while cur != start.0 {
                if vertex_set.contains(&cur) {
                    return Err(anyhow::Error::msg("Vertex path already exists, this is algorithm mistake, report this with used example"));
                }
                // next and prev to cur in terms of global polygon
                idx_next        = (cur + 1) % N;
                idx_prev        = (cur + N - 1) % N;
                // next_cur_vec    = points[idx_next] - points[cur];
                last_edge       = points[*(vertexpath.last().unwrap())] - points[cur];
                // check_edge      = next_cur_vec.clone();
                check = last_edge * check_edge;
                let (_, _, next_cur_id) = {
                    [
                        vec![idx_next, idx_prev ],
                        if let Some(v) = extra_segments.get(&cur) { v.clone() } else { Vec::new() },
                    ].concat().iter()
                        .filter(|&x| {
                            !vertex_set.contains(x) &&
                            match self.used.get(&(cur, *x)) {
                                None => { true }
                                Some(t) => {
                                    *t < 1 || *t < 2 && {
                                        match extra_segments.get(&cur) {
                                            None => { false }
                                            Some(pts) => {
                                                pts.contains(x) // cheks if &(cur, *x) is extra added
                                                // bcs, not added edges are one-time-usable
                                            }
                                        }
                                    }
                                }
                            }
                        })
                        .map(
                            |this| {
                                check_edge    = points[*this] - points[cur];
                                last_edge.z = 0.0;
                                check_edge.z = 0.0;
                                check = last_edge * check_edge;
                                (check.length() / (last_edge.length() * check_edge.length()), check.z > 0.0, *this)
                            }
                        ).filter(
                            |x| {
                                x.1 == is_right
                            }
                        ).fold(
                            (f32::INFINITY, false, usize::MAX),
                            |state, val| {
                                // println!("Predictors: of {} is {:?}", cur, val);
                                if state.0 > val.0 {
                                    val.clone()
                                } else {
                                    val
                                }
                            }
                        )
                };
                if next_cur_id == usize::MAX {
                    if (cur + 1) % N == start.0 || {
                        match extra_segments.get(&cur) {
                            None => { false }
                            Some(t) => { t.contains(&start.0) }
                        }
                    } {
                        // its final!
                        vertexpath.push(cur);
                        vertex_set.insert(cur);
                        break;
                    } else {
                        return Err(anyhow::Error::msg(format!("No such new polygon (next segments) when searching for: {}", cur)));
                    }
                }
                vertexpath.push(cur);
                vertex_set.insert(cur);
                cur = next_cur_id;
            }
            let M = vertexpath.len();
            let mut ii;
            for i in 0..M {
                ii = (i + 1) % M;
                let x = self.used.entry((vertexpath[i], vertexpath[ii]) ).or_insert(0);
                *x += 1;
            }
            Ok(vertexpath)
        }
    }
}

fn to_monotonic_addings (
    points: &[Vec3]
) -> anyhow::Result<HashMap<usize, Vec<usize>> >{
    let mut ysorted = (0..points.len()).collect::<Vec<_>>();
    ysorted.sort_by(
        |i, j| {
            if points[*i].y != points[*j].y {
                points[*i].y.partial_cmp(&points[*j].y).unwrap()
            } else {
                points[*j].x.partial_cmp(&points[*i].x).unwrap()
            }


        }
    );
    ysorted.reverse();
    let N = points.len();
    let mut helpers = vec![(0 as usize, false); N]; // false is merge
    let mut T: Option<SegmentBinTree> = None;
    // let mut run: Vec<usize> = vec![];
    let mut double_edge: HashMap<usize, Vec<usize>> = HashMap::new();
    //println!("{:?}", ysorted);
    for i in ysorted.iter() {
        let idx_next = (i + 1) % N;
        let idx_prev = (i + N - 1) % N;
        let next = &points[idx_next];
        let prev = &points[idx_next];
        let this = &points[*i];
        let vn = next - this;
        let vp = prev - this;
        let angle = vn.angle_between(vp);
        if next.y > this.y && prev.y > this.y {
            // both are up (above)
            if angle < PI {
                // end
                //println!("{} {:?} is end", *i, this);
                if helpers[ idx_prev ].1 {
                    // connect this to helper[idx_prev]
                    // extra.push((*i, idx_prev));
                    double_edge.entry(*i).or_insert(Vec::new()).push( helpers[idx_prev].0 );
                    double_edge.entry( helpers[idx_prev].0 ).or_insert(Vec::new()).push(*i);
                }
                match &mut T {
                    None => {
                        return Err(anyhow::Error::msg("Tree processing error (end)"));
                    }
                    Some(t) => {
                        t.remove(idx_prev, prev.x)?;
                    }
                }
            } else {
                // merge
                //println!("{} {:?} is merge", *i, this);
                if helpers[idx_prev].1 { // is merge
                    // connect this to prev
                    // extra.push((*i, idx_prev));
                    double_edge.entry(*i).or_insert(Vec::new()).push(idx_prev);
                    double_edge.entry(idx_prev).or_insert(Vec::new()).push(*i);
                }
                let k = match &mut T {
                    None => {
                        return Err(anyhow::Error::msg("Tree processing error (merge)"));
                    }
                    Some(t) => {
                        // remove prev from tree
                        t.remove(idx_prev, prev.x)?;
                        // search in tree to find K: e_k directly left to this
                        t.nearest_left(this.x, this.y, points, N)?
                    }
                };
                if helpers[k].1 {
                    // connect this to helper[k]
                    // extra.push((*i, helpers[k].0));
                    double_edge.entry(*i).or_insert(Vec::new()).push(helpers[k].0);
                    double_edge.entry(helpers[k].0).or_insert(Vec::new()).push(*i);
                }
                helpers[k] = (*i, true);
            }
        } else if next.y < this.y && prev.y < this.y {
            // both are down (below)
            if angle < PI {
                // start
                //println!("{} {:?} is start", *i, this);
                helpers[*i] = (*i, false);
                match &mut T {
                    None => {
                        // return Err(anyhow::Error::msg("Tree processing error (start)"));
                        T = Some(SegmentBinTree::new(*i, this.x));
                    }
                    Some(t) => {
                        t.insert(
                            SegmentBinTree::new(*i, this.x)
                        )?;
                    }
                }
            } else {
                // split
                // println!("{} {:?} is split", *i, this);
                let k;
                match &mut T {
                    None => {
                        return Err(anyhow::Error::msg("Tree processing error (split)"));
                    }
                    Some(t) => {
                        // search in tree to find K: e_k directly left to this
                        k = t.nearest_left(this.x, this.y, &points, N)?;
                        // add this to tree
                        t.insert(
                            SegmentBinTree::new(*i, this.x)
                        )?;
                    }
                }; // k is nearest left id
                // connect this to helper[e_k]
                // extra.push((*i, helpers[k].0));
                double_edge.entry(*i).or_insert(Vec::new()).push(helpers[k].0);
                double_edge.entry(helpers[k].0).or_insert(Vec::new()).push(*i);
                helpers[k] = (*i, false);
                helpers[*i] = (*i, false);
            }
        } else {
            // else - regular
            if prev.y < this.y || (prev.y == this.y && prev.x > this.x) {
                // left
                if helpers[idx_prev].1 {
                    // prev is merge
                    // extra.push((*i, helpers[idx_prev].0));
                    double_edge.entry(*i).or_insert(Vec::new()).push(helpers[idx_prev].0);
                    double_edge.entry(helpers[idx_prev].0).or_insert(Vec::new()).push(*i);
                    // remove prev from tree
                    // insert this to tree
                    match &mut T {
                        None => {
                            return Err(anyhow::Error::msg("Tree processing error"));
                        }
                        Some(t) => {
                            t.remove(idx_prev, prev.x)?;
                            t.insert(
                                SegmentBinTree::new(*i, this.x)
                            )?;
                        }
                    }
                    //
                    helpers[*i] = (*i, false);
                } else {
                    let k = match &mut T {
                        None => {
                            return Err(anyhow::Error::msg("Tree processing error"));
                        }
                        Some(t) => {
                            // search in tree to find K: e_k directly left to this
                            let k =t.nearest_left(this.x, this.y, &points, N)?;
                            // add this to tree
                            t.insert(
                                SegmentBinTree::new(*i, this.x)
                            )?;
                            k
                        }
                    };
                    if helpers[k].1 {
                        // k is merge
                        // extra.push((*i, helpers[k].0));
                        double_edge.entry(*i).or_insert(Vec::new()).push(helpers[k].0);
                        double_edge.entry(helpers[k].0).or_insert(Vec::new()).push(*i);

                    }
                    helpers[k] = (*i, false);
                }
            } else {
                // right
                // PASS
            }
        }
    }
    Ok(double_edge)
}

pub fn triangulate_2d(
    points: &[Vec3],
    // index_offset: usize,
) -> anyhow::Result< Vec<u32> > {
    if points.len() <= 2 {
        Err(anyhow::Error::msg("not enough points"))
    } else {
        // CONVERT TO Y-MONOTONIC POLYGONS
        let double_edge = match to_monotonic_addings(points) {
            Ok(d) => {d}
            Err(e) => {
                // println!("To monotonic response error: {}", e);
                // TODO logging
                Default::default()
            }
        };
        // TRIANGULATION
        // println!(">>>> EXTRA SEGMENTS {:?}", double_edge);
        let mut pol_indices = Vec::new();
        let mut selector = PolygonSelector::new();
        let mut __n = HashMap::<usize, Vec<usize>>::default();
        __n.insert(0usize, vec![1usize]);
        for i in {
            if double_edge.len() > 0 {
                double_edge.iter()
            } else {
                __n.iter()
            }
        }
        {
            for j in i.1 {
                // println!(">>>> Start segment {} to {}", i.0, j);
                match selector.select((*i.0, *j), &points, &double_edge) {
                    Ok(monotonic_pol_points) => {
                        // triangulate this polygon
                        //println!("Polygon indeces detected: {:?}", monotonic_pol_points);
                        let sorted = {
                            let mut t = (0..monotonic_pol_points.len()).collect::<Vec<_>>();
                            t.sort_by(
                                |i, j| {
                                    let a = monotonic_pol_points[*i];
                                    let b = monotonic_pol_points[*j];
                                    if points[a].y != points[b].y {
                                        points[a].y.partial_cmp(&points[b].y).unwrap()
                                    } else {
                                        points[b].x.partial_cmp(&points[a].x).unwrap()
                                    }
                                }
                            );
                            t.reverse();
                            t
                        };
                        let mut stack = Vec::new();
                        stack.push(sorted[0]);
                        stack.push(sorted[1]);
                        let M = monotonic_pol_points.len();
                        let mut prev_k;
                        let mut last_k = sorted[1];
                        for k in sorted[2..sorted.len()-1].iter() {
                            //println!("STACK: on k {} -> {:?}", k, stack);
                            prev_k = (k + M - 1) % M;
                            if let Some(top) = stack.pop() {
                                let prev_top = (top + M - 1) % M;
                                if
                                    (points[monotonic_pol_points[*k]].y.partial_cmp(&points[monotonic_pol_points[prev_k]].y)) !=
                                        (points[monotonic_pol_points[top]].y.partial_cmp(&points[monotonic_pol_points[prev_top]].y))
                                    // || (
                                    //     // (points[monotonic_pol_points[*k]].y.partial_cmp(&points[monotonic_pol_points[prev_k]].y)) ==
                                    //     //     (points[monotonic_pol_points[top]].y.partial_cmp(&points[monotonic_pol_points[prev_top]].y)) &&
                                    //     (points[monotonic_pol_points[*k]].x.partial_cmp(&points[monotonic_pol_points[prev_k]].x)) !=
                                    //         (points[monotonic_pol_points[top]].x.partial_cmp(&points[monotonic_pol_points[prev_top]].x))
                                    // )
                                {
                                    stack.push(top);
                                    // different chains
                                    let mut u = stack.len() - 1;
                                     while u != 0 {
                                         let n0_idx = monotonic_pol_points[*k];
                                         // println!(
                                         //     "DIF k: {}; prev k: {}; top: {}; prev_top: {}; {:?} :: {:?}", *k, prev_k, top, prev_top, stack, sorted
                                         // );
                                        pol_indices.push(
                                            n0_idx as u32
                                        );
                                         let n1_idx = monotonic_pol_points[stack[u]];
                                         // let n2_idx = monotonic_pol_points[stack[u-1]];
                                         let n2_idx = monotonic_pol_points[stack[u - 1]];
                                         if (
                                             {
                                                 (points[n1_idx].x - points[n0_idx].x) * (points[n1_idx].y + points[n0_idx].y) +
                                                 (points[n2_idx].x - points[n1_idx].x) * (points[n2_idx].y + points[n1_idx].y) +
                                                 (points[n0_idx].x - points[n2_idx].x) * (points[n0_idx].y + points[n2_idx].y)
                                             } < 0.0
                                         ) {
                                             pol_indices.push(
                                                 n1_idx as u32
                                             );
                                             pol_indices.push(
                                                 n2_idx as u32
                                             );
                                         } else {
                                             pol_indices.push(
                                                 n2_idx as u32
                                             );
                                             pol_indices.push(
                                                 n1_idx as u32
                                             );
                                         }
                                        //pol_indices.push(
                                        //    monotonic_pol_points[stack[u]] as u32
                                        //);
                                        //pol_indices.push(
                                        //    monotonic_pol_points[stack[u-1]] as u32
                                        //);
                                         u -= 1;
                                    }
                                    stack.clear();
                                    stack.push(last_k);
                                    stack.push(*k);
                                } else {
                                    // same chain
                                    let mut last_popped = top;
                                    let mut popped = 0usize;
                                    let is_inside = |f: usize, b: usize, t: usize, is_from_left: bool| -> bool {
                                        let fb = points[monotonic_pol_points[f]] - points[monotonic_pol_points[b]];
                                        let tb = points[monotonic_pol_points[t]] - points[monotonic_pol_points[b]];
                                        let cr = fb.x*tb.y - fb.y*tb.x;
                                        if is_from_left {
                                            cr > 0.0
                                        } else {
                                            cr < 0.0
                                        }
                                    };
                                    while !stack.is_empty() {
                                        popped = stack.pop().unwrap();
                                        if last_popped == top || is_inside(*k, popped, last_popped, {
                                            //(points[monotonic_pol_points[*k]].y.partial_cmp(&points[monotonic_pol_points[prev_k]].y));
                                            //(points[monotonic_pol_points[*k]].x.partial_cmp(&points[monotonic_pol_points[prev_k]].x));
                                            if points[monotonic_pol_points[*k]].y != points[monotonic_pol_points[prev_k]].y {
                                                points[monotonic_pol_points[*k]].y < points[monotonic_pol_points[prev_k]].y
                                            } else {
                                                points[monotonic_pol_points[prev_k]].x < points[monotonic_pol_points[*k]].x
                                            }
                                        })
                                        {
                                            let n0_idx = monotonic_pol_points[*k];
                                            pol_indices.push(
                                                n0_idx as u32
                                            );
                                            // println!(
                                            //     "SAME {} {:?}", n0_idx, stack
                                            // );
                                            let n1_idx = monotonic_pol_points[last_popped];
                                            let n2_idx = monotonic_pol_points[popped];
                                            if (
                                                {
                                                    (points[n1_idx].x - points[n0_idx].x) * (points[n1_idx].y + points[n0_idx].y) +
                                                        (points[n2_idx].x - points[n1_idx].x) * (points[n2_idx].y + points[n1_idx].y) +
                                                        (points[n0_idx].x - points[n2_idx].x) * (points[n0_idx].y + points[n2_idx].y)
                                                } < 0.0
                                            ) {
                                                pol_indices.push(
                                                    n1_idx as u32
                                                );
                                                pol_indices.push(
                                                    n2_idx as u32
                                                );
                                            } else {
                                                pol_indices.push(
                                                    n2_idx as u32
                                                );
                                                pol_indices.push(
                                                    n1_idx as u32
                                                );
                                            }
                                            // pol_indices.push(
                                            //     monotonic_pol_points[last_popped] as u32
                                            // );
                                            // pol_indices.push(
                                            //     monotonic_pol_points[popped] as u32
                                            // );

                                            last_popped = popped;
                                        } else {
                                            stack.push(popped);
                                            popped = last_popped;
                                            break;
                                        }
                                    }
                                    stack.push(popped);
                                    stack.push(*k);
                                }
                            } else {
                                // ERR
                                return Err(anyhow::Error::msg("Stack is empty"))
                            }
                            last_k = *k;
                        }
                        let fr_diag = *sorted.last().unwrap();
                        //println!("Final stack {:?}", stack);
                        for i in 1..stack.len() {
                            //println!("i: {}", i);
                            let n0_idx = monotonic_pol_points[fr_diag];
                            pol_indices.push(
                                n0_idx as u32
                            );
                            let n1_idx = monotonic_pol_points[stack[i]];
                            let n2_idx = monotonic_pol_points[stack[i-1]];
                            if (
                                {
                                    (points[n1_idx].x - points[n0_idx].x) * (points[n1_idx].y + points[n0_idx].y) +
                                        (points[n2_idx].x - points[n1_idx].x) * (points[n2_idx].y + points[n1_idx].y) +
                                        (points[n0_idx].x - points[n2_idx].x) * (points[n0_idx].y + points[n2_idx].y)
                                } < 0.0
                            ) {
                                pol_indices.push(
                                    n1_idx as u32
                                );
                                pol_indices.push(
                                    n2_idx as u32
                                );
                            } else {
                                pol_indices.push(
                                    n2_idx as u32
                                );
                                pol_indices.push(
                                    n1_idx as u32
                                );
                            }
                            // println!("last added: {} {} {}",
                            //         fr_diag, monotonic_pol_points[stack[i]], monotonic_pol_points[stack[i-1]]
                            // )
                        }
                    }
                    Err(e) => {
                        println!(">>>> {:?}", e);
                    }
                }
            }
        }
        Ok(pol_indices)
    }
}

#[test]
fn test_polygon() {
    let v = vec![
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(0.5, 1.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];
    let res = triangulate_2d(
        &v
    );
    println!("{:?}", res);


}