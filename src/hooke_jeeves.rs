use crate::{add, basis};

fn hooke_jeeves<const N: usize, F: Fn([f32; N]) -> f32>(
    f: F,
    mut x: [f32; N],
    mut alpha: f32,
    epsilon: f32,
    gamma: f32,
) -> [f32; N] {
    let mut y = f(x);

    while alpha > epsilon {
        let mut improved = false;
        let (mut x_best, mut y_best) = (x, y);
        for i in 0..N {
            for sgn in [-1., 1.] {
                let x_prime = add(x, basis::<N>(i).map(|b| b * sgn * alpha));
                let y_prime = f(x_prime);
                if y_prime < y_best {
                    (x_best, y_best, improved) = (x_prime, y_prime, true);
                }
            }
        }
        (x, y) = (x_best, y_best);

        if !improved {
            alpha *= gamma;
        }
    }
    x
}
