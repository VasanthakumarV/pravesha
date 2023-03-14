struct MethodConfig {
    bracket: Interval,
    lipschitz_const: f32,
    epsilon: f32,
    delta: f32,
}

#[derive(Clone, Copy)]
struct Interval {
    left: f32,
    right: f32,
}

impl From<(f32, f32)> for Interval {
    fn from((left, right): (f32, f32)) -> Self {
        Self { left, right }
    }
}

#[derive(Clone, Copy)]
struct Pt {
    x: f32,
    y: f32,
}

impl Pt {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn shubert_piyavski_method(f: impl Fn(f32) -> f32, config: MethodConfig) -> (Pt, Vec<Interval>) {
    let MethodConfig {
        bracket: Interval { left, right },
        lipschitz_const: l,
        epsilon,
        delta,
    } = config;

    let middle = (left + right) / 2.;
    let (a, m, b) = (
        Pt::new(left, f(left)),
        Pt::new(middle, f(middle)),
        Pt::new(right, f(right)),
    );
    let mut pts = vec![
        a,
        get_intersection(a, m, l),
        m,
        get_intersection(m, b, l),
        b,
    ];

    let mut diff = f32::INFINITY;
    while diff > epsilon {
        let i = argmin(pts.iter().map(|p| p.y));
        let p = Pt::new(pts[i].x, f(pts[i].x));
        let p_prev = get_intersection(pts[i - 1], p, l);
        let p_next = get_intersection(p, pts[i + 1], l);

        diff = p.y - pts[i].y;

        pts.remove(i);
        pts.insert(i, p_next);
        pts.insert(i, p);
        pts.insert(i, p_prev);
    }

    let mut intervals: Vec<Interval> = vec![];
    let i = 2 * argmin(pts.iter().step_by(2).map(|p| p.y)) - 1;
    pts.iter().skip(1).step_by(2).for_each(|p| {
        if p.y < pts[i].y {
            let dy = pts[i].y - p.y;
            let x_lo = left.max(p.x - dy / l);
            let x_hi = right.min(p.x + dy / l);
            if (!intervals.is_empty())
                && (intervals.last().map(|i| i.right + delta).unwrap() >= x_lo)
            {
                if let Some(i) = intervals.last_mut() {
                    *i = (i.left, x_hi).into();
                }
            } else {
                intervals.push((x_lo, x_hi).into());
            }
        }
    });

    (pts[i], intervals)
}

fn get_intersection(left: Pt, right: Pt, l: f32) -> Pt {
    let t = ((left.y - right.y) - l * (left.x - right.x)) / (2. * l);
    Pt::new(left.x + t, left.y - t * l)
}

fn argmin(vals: impl Iterator<Item = f32>) -> usize {
    vals.enumerate()
        .fold((0, 0.), |(idx_max, val_max), (idx, v)| {
            if val_max > v {
                (idx_max, val_max)
            } else {
                (idx, v)
            }
        })
        .0
}
