fn line_search<const N: usize>(
    f: impl Fn([f32; N]) -> f32,
    x: [f32; N],
    dir: [f32; N],
) -> [f32; N] {
    let objective = |alpha: f32| {
        let inp = x.zip(dir).map(|(x, d)| x + alpha * d);
        f(inp)
    };

    let (a, b) = todo!("bracket_minimum(objective)");
    let alpha = todo!("minimize(objective, a, b)");

    x.zip(dir).map(|(x, d)| x + alpha * d)
}
