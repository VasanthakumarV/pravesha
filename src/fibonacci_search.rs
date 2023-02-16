struct SearchConfig {
    bracket: Interval,
    n_queries: usize,
    eps: f32,
}

#[derive(Debug)]
struct Interval {
    start: f32,
    end: f32,
}

impl From<(f32, f32)> for Interval {
    fn from((start, end): (f32, f32)) -> Self {
        Self { start, end }
    }
}

fn fibonacci_ratio(n: i32) -> f32 {
    let phi = (1. + 5f32.sqrt()) / 2.;
    let s = (1. - 5f32.sqrt()) / (1. + 5f32.sqrt());
    phi * (1. - s.powi(n + 1)) / (1. - s.powi(n))
}

fn fibonacci_search<F: Fn(f32) -> f32>(f: F, config: SearchConfig) -> Interval {
    let SearchConfig {
        bracket: Interval {
            start: mut a,
            end: mut b,
        },
        n_queries,
        eps,
    } = config;

    let mut rho = 1. / fibonacci_ratio(n_queries as i32);
    let mut d = rho * b + (1. - rho) * a;
    let mut yd = f(d);

    (1..n_queries - 1).for_each(|i| {
        let c = if i == n_queries - 1 {
            eps * a + (1. - eps) * d
        } else {
            rho * a + (1. - rho) * b
        };

        let yc = f(c);

        if yc < yd {
            (b, d, yd) = (d, c, yc);
        } else {
            (a, b) = (b, c);
        }

        rho = 1. / fibonacci_ratio(n_queries as i32 - 1);
    });

    if a < b {
        (a, b).into()
    } else {
        (b, a).into()
    }
}
