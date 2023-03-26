use crate::add;

fn generalized_pattern_search<const N: usize, F: Fn([f32; N]) -> f32>(
    f: F,
    mut x: [f32; N],
    mut alpha: f32,
    mut dirs: Vec<[f32; N]>,
    epsilon: f32,
    gamma: f32,
) -> [f32; N] {
    let mut y = f(x);
    while alpha > epsilon {
        let mut improved = false;
        for (i, dir) in dirs.iter().enumerate() {
            let x_prime = add(x, dir.map(|d| d * alpha));
            let y_prime = f(x_prime);
            if y_prime < y {
                (x, y, improved) = (x_prime, y_prime, true);
                dirs.remove(i);
                dirs.insert(0, *dir);
                break;
            }
        }
        if !improved {
            alpha *= gamma;
        }
    }
    x
}
