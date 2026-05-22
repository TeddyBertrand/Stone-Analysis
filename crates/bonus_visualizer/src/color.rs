#[derive(Clone, Copy)]
pub struct Rgb(pub u8, pub u8, pub u8);

pub fn heat_colormap(t: f32) -> Rgb {
    const STOPS: &[(f32, Rgb)] = &[
        (0.00, Rgb(  0,   0,   0)),
        (0.25, Rgb(  0,   0, 180)),
        (0.50, Rgb(  0, 200, 200)),
        (0.75, Rgb(255, 220,   0)),
        (1.00, Rgb(255, 255, 255)),
    ];

    let t = t.clamp(0.0, 1.0);
    for i in 0..STOPS.len() - 1 {
        let (t0, c0) = STOPS[i];
        let (t1, c1) = STOPS[i + 1];
        if t <= t1 {
            let a = (t - t0) / (t1 - t0);
            return Rgb(lerp(c0.0, c1.0, a), lerp(c0.1, c1.1, a), lerp(c0.2, c1.2, a));
        }
    }
    STOPS.last().unwrap().1
}

#[inline]
fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t) as u8
}