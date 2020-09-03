use rustfft::num_complex::Complex;

pub fn soulti_cpx(t: f32, zeta: f32) -> Complex<f32> {
    let k: f32 = 1.0 - zeta.powi(2);
    const TAU: f32 = std::f32::consts::PI * 2.0;

    if t > 0.0 {
        (-zeta / k * TAU * t).exp() * (TAU * t * Complex::i()).exp() / k
    } else {
        Complex::new(0., 0.)
    }
}

pub fn soulti(t: f32, zeta: f32) -> f32 {
    let k: f32 = 1.0 - zeta.powi(2);
    const TAU: f32 = std::f32::consts::PI * 2.0;

    if t > 0.0 {
        (-zeta / k * TAU * t).exp() * (TAU * t).sin() / k
    } else {
        0.0
    }
}