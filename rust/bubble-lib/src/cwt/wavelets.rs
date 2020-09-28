use rustfft::num_complex::Complex;
use std::f32::consts::TAU;

pub enum WaveletFn {
    Soulti(Soulti),
    Morlet(Morlet),
}

impl WaveletFn {
    pub fn func(&self, t: f32) -> Complex<f32> {
        match self {
            WaveletFn::Morlet(m) => m.func(t),
            WaveletFn::Soulti(s) => s.func(t),
        }
    }
}

// Soulti
pub struct Soulti {
    a: f32,
    b: f32,
}

impl Soulti {
    pub fn new(zeta: f32) -> Self {
        let k = 1.0 - zeta.powi(2);
        Soulti {
            a: -zeta / k * TAU,
            b: k.recip(),
        }
    }
    fn func(&self, t: f32) -> Complex<f32> {
        if t > 0.0 {
            (self.a * t).exp() * (TAU * t * Complex::i()).exp() * self.b
        } else {
            Complex::new(0., 0.)
        }
    }
}

// Morlet
pub struct Morlet {}

impl Morlet {
    pub fn new() -> Self {
        Morlet {}
    }
    fn func(&self, t: f32) -> Complex<f32> {
        (-0.5 * (t * TAU * 0.2).powi(2)).exp() * (TAU * t * Complex::i()).exp()
    }
}


// will be removed later v
pub fn soulti(t: f32, zeta: f32) -> f32 {
    let k: f32 = 1.0 - zeta.powi(2);
    const TAU: f32 = std::f32::consts::PI * 2.0;

    if t > 0.0 {
        (-zeta / k * TAU * t).exp() * (TAU * t).sin() / k
    } else {
        0.0
    }
}
