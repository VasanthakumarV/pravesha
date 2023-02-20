struct SearchConfig {
    bracket: Interval,
    n_queries: usize,
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

fn golden_section_search<F: Fn(f32) -> f32>(f: F, config: SearchConfig) -> Interval {
    let SearchConfig {
        bracket: Interval {
            start: mut a,
            end: mut b,
        },
        n_queries,
    } = config;

    let rho = 1.618 - 1.;
    let mut d = rho * b + (1. - rho) * a;
    let mut yd = f(d);

    for _ in 1..n_queries - 1 {
        let c = rho * a + (1. - rho) * b;
        let yc = f(c);

        if yc < yd {
            (b, d, yd) = (d, c, yc);
        } else {
            (a, b) = (b, c);
        }
    }

    if a < b {
        (a, b).into()
    } else {
        (b, a).into()
    }
}
