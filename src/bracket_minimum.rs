struct BracketConfig {
    start: f32,
    step_size: f32,
    expansion: f32,
}

#[derive(Debug)]
struct Interval {
    start: f32,
    end: f32,
}

impl From<(f32, f32)> for Interval {
    fn from((start, end): (f32, f32)) -> Self {
        Interval { start, end }
    }
}

fn bracket_minimum(f: impl Fn(f32) -> f32, config: BracketConfig) -> Interval {
    let BracketConfig {
        start,
        mut step_size,
        expansion,
    } = config;

    let (mut a, ya) = (start, f(start));
    let (mut b, mut yb) = (a + step_size, f(a + step_size));

    if yb > ya {
        (a, b, yb) = (b, a, ya);
        step_size = -step_size;
    }

    loop {
        let (c, yc) = (b + step_size, f(b + step_size));
        if yc > yb {
            return if a < c { (a, c).into() } else { (c, a).into() };
        }
        (a, b, yb) = (b, c, yc);
        step_size *= expansion;
    }
}
