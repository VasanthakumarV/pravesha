use crate::{add, sub};

struct NelderMeadConfig {
    alpha: f32,
    beta: f32,
    gamma: f32,
}

fn nelder_mead<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    mut s: [[f32; N]; N + 1],
    epsilon: f32,
    config: NelderMeadConfig,
) -> [f32; N] {
    let NelderMeadConfig { alpha, beta, gamma } = config;

    let delta = f32::INFINITY;
    let mut y_arr = s.map(|s| f(s));
    while delta > epsilon {
        let p = argsort(y_arr);
        (s, y_arr) = (p.map(|i| s[i]), p.map(|i| y_arr[i]));
        let (xl, yl) = (s[0], y_arr[0]);
        let (xh, yh) = (s[N], y_arr[N]);
        let (xs, ys) = (s[N - 1], y_arr[N - 1]);
        let xm = s[0..N]
            .iter()
            .fold([0.; N], |acc, x| acc.zip(*x).map(|(a, x)| a + x))
            .map(|x| x / N as f32);
        let xr = add(xm, sub(xm, xh).map(|x| x * alpha));
        let yr = f(xr);

        if yr < yl {
            let xe = add(xm, sub(xr, xm).map(|x| x * beta));
            let ye = f(xe);
            (s[N], y_arr[N]) = if ye < yr { (xe, ye) } else { (xr, yr) };
        } else if yr > ys {
            if yr <= yh {
                (xh, yh, s[N], y_arr[N]) = (xr, yr, xr, yr);
            }
            let xc = add(xm, sub(xh, xm).map(|x| x * gamma));
            let yc = f(xc);
            if yc > yh {
                for i in 1..=N {
                    s[i] = add(s[i], xl).map(|x| x / 2.);
                    y_arr[i] = f(s[i]);
                }
            } else {
                (s[N], y_arr[N]) = (xc, yc);
            }
        } else {
            (s[N], y_arr[N]) = (xr, yr);
        }

        delta = std_dev(&y_arr);
    }

    s[argmin(&y_arr)]
}

fn argsort<const N: usize>(arr: [f32; N]) -> [usize; N] {
    let mut indices = [0; N];
    for i in 0..N {
        indices[i] = i;
    }
    indices.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    indices
}

fn argmin(arr: &[f32]) -> usize {
    arr.iter()
        .enumerate()
        .fold((0, f32::INFINITY), |(mut min_idx, mut min), (i, &a)| {
            if a < min {
                (min_idx, min) = (i, a);
            }
            (min_idx, min)
        })
        .0
}

fn std_dev(arr: &[f32]) -> f32 {
    let n = arr.len() as f32;
    let m = arr.iter().sum::<f32>() / n;
    let variance = arr.iter().map(|x| (x - m).powi(2)).sum::<f32>() / n;
    variance.sqrt()
}
