fn secant_method(grad_f: impl Fn(f32) -> f32, x0: f32, x1: f32, epsilon: f32) -> f32 {
    let g0 = grad_f(x0);
    let mut delta = f32::INFINITY;
    while delta.abs() > epsilon {
        let g1 = grad_f(x1);
        delta = (x1 - x0) / (g1 - g0) * g1;
        (x0, x1, g0) = (x1, x1 - delta, g1);
    }
    x1
}
