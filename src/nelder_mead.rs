use crate::{add, sub};

struct NelderMeadConfig {
    alpha: f32,
    beta: f32,
    gamma: f32,
}

fn nelder_mead(
    f: impl Fn([f32; 2]) -> f32,
    mut s: [[f32; 2]; 3],
    epsilon: f32,
    config: NelderMeadConfig,
) -> [f32; 2] {
    let NelderMeadConfig { alpha, beta, gamma } = config;

    let delta = f32::INFINITY;
    let mut y_arr = s.map(|s| f(s));
    while delta > epsilon {
        let p = argsort(y_arr);
        (s, y_arr) = (p.map(|i| s[i]), p.map(|i| y_arr[i]));
        let (xl, yl) = (s[0], y_arr[0]);
        let (xh, yh) = (s[2], y_arr[2]);
        let (xs, ys) = (s[1], y_arr[1]);
        let xm = add(xl, xs).map(|x| x / 2.);
        let xr = add(xm, sub(xm, xh).map(|x| x * alpha));
        let yr = f(xr);

        if yr < yl {
            let xe = add(xm, sub(xr, xm).map(|x| x * beta));
            let ye = f(xe);
            (s[2], y_arr[2]) = if ye < yr { (xe, ye) } else { (xr, yr) };
        } else if yr > ys {
            if yr <= yh {
                (xh, yh, s[2], y_arr[2]) = (xr, yr, xr, yr);
            }
            let xc = add(xm, sub(xh, xm).map(|x| x * gamma));
            let yc = f(xc);
            if yc > yh {
                for i in 1..3 {
                    s[i] = add(s[i], xl).map(|x| x / 2.);
                    y_arr[i] = f(s[i]);
                }
            } else {
                (s[2], y_arr[2]) = (xc, yc);
            }
        } else {
            (s[2], y_arr[2]) = (xr, yr);
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
