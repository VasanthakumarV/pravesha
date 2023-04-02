use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use crate::{add, basis, mul, sub};

#[derive(Clone, Copy)]
struct Interval<const N: usize> {
    c: [f32; N],
    y: f32,
    depths: [usize; N],
}

impl<const N: usize> Interval<N> {
    fn new(c: [f32; N], y: f32, depths: [usize; N]) -> Self {
        Self { c, y, depths }
    }
}

impl<const N: usize> PartialEq for Interval<N> {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y
    }
}

impl<const N: usize> Eq for Interval<N> {}

impl<const N: usize> PartialOrd for Interval<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.y.partial_cmp(&other.y).unwrap().reverse())
    }
}

impl<const N: usize> Ord for Interval<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

type Intervals<const N: usize> = HashMap<usize, BinaryHeap<Interval<N>>>;

fn direct<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    a: [f32; N],
    b: [f32; N],
    epsilon: f32,
    max_iter: usize,
) -> [f32; N] {
    let g = reparam(f, a, b);
    let mut intervals: Intervals<N> = HashMap::new();
    let c = [0.5; N];
    let interval = Interval::new(c, g(c), [0; N]);
    add_interval(&mut intervals, interval);
    let (mut c_best, mut y_best) = (interval.c, interval.y);

    for k in 0..max_iter {
        let s = get_opt_intervals(intervals, epsilon, y_best);
        let mut to_add = Vec::new();
        for interval in s {
            to_add.append(&mut divide(g, interval));
            intervals.remove(&min_depth(&interval));
        }
        for interval in to_add {
            add_interval(&mut intervals, interval);
            if interval.y < y_best {
                (c_best, y_best) = (interval.c, interval.y);
            }
        }
    }

    add(mul(c_best, sub(b, a)), a)
}

fn get_opt_intervals<const N: usize>(
    intervals: Intervals<N>,
    epsilon: f32,
    y_best: f32,
) -> Vec<Interval<N>> {
    let max_depth = intervals.keys().max().unwrap();
    let mut stack = vec![*intervals[max_depth].peek().unwrap()];
    let mut d = max_depth - 1;

    while d >= 0 {
        match intervals.get(&d) {
            Some(pq) if !pq.is_empty() => {
                let interval = pq.peek().unwrap();
                let (x, y) = (0.5 * 3f32.powi(-(min_depth(interval) as i32)), interval.y);

                while !stack.is_empty() {
                    let interval_1 = stack.last().unwrap();
                    let x1 = 0.5 * 3f32.powi(-(min_depth(interval) as i32));
                    let y1 = interval_1.y;
                    let l1 = (y - y1) / (x - x1);
                    if y1 - l1 * x1 > y_best || y < y1 {
                        stack.pop();
                    } else if stack.len() > 1 {
                        let interval_2 = stack[stack.len() - 2];
                        let x2 = 0.5 * 3f32.powi(-(min_depth(&interval_2) as i32));
                        let y2 = interval_2.y;
                        let l2 = (y1 - y2) / (x1 - x2);
                        if l2 > l1 {
                            stack.pop();
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            _ => {}
        }
        d -= 1;
    }

    stack
}

fn divide<const N: usize>(f: impl Fn([f32; N]) -> f32, interval: Interval<N>) -> Vec<Interval<N>> {
    let (c, d) = (interval.c, min_depth(&interval));
    let dirs = interval
        .depths
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| if x == d { Some(i) } else { None })
        .collect::<Vec<_>>();
    let cs = dirs
        .into_iter()
        .map(|i| {
            (
                add(c, basis::<N>(i).map(|b| 3f32.powi(-(d as i32) - 1) * b)),
                sub(c, basis::<N>(i).map(|b| 3f32.powi(-(d as i32) - 1) * b)),
            )
        })
        .collect::<Vec<_>>();
    let vs = cs
        .into_iter()
        .map(|(c1, c2)| (f(c1), f(c2)))
        .collect::<Vec<_>>();
    let min_vals = vs
        .into_iter()
        .map(|(v1, v2)| v1.min(v2))
        .collect::<Vec<_>>();

    let mut intervals = Vec::new();
    let mut depths = interval.depths;
    let mut sort_perm: Vec<_> = (0..min_vals.len()).collect();
    sort_perm.sort_unstable_by(|&a, &b| min_vals[a].partial_cmp(&min_vals[b]).unwrap());
    for j in sort_perm {
        depths[dirs[j]] += 1;
        let ((c1, c2), (v1, v2)) = (cs[j], vs[j]);
        intervals.push(Interval::new(c1, v1, depths));
        intervals.push(Interval::new(c2, v2, depths));
    }
    intervals.push(Interval::new(c, interval.y, depths));
    intervals
}

fn add_interval<const N: usize>(intervals: &mut Intervals<N>, interval: Interval<N>) {
    let d = min_depth(&interval);
    if let Some(queue) = intervals.get_mut(&d) {
        queue.push(interval);
    } else {
        intervals.insert(d, BinaryHeap::from([interval]));
    }
}

fn min_depth<const N: usize>(interval: &Interval<N>) -> usize {
    interval.depths.into_iter().min().unwrap()
}

fn reparam<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    a: [f32; N],
    b: [f32; N],
) -> impl Fn([f32; N]) -> f32 {
    let delta = sub(b, a);
    move |x: [f32; N]| f(add(mul(x, delta), a))
}
