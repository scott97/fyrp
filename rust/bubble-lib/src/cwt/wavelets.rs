use rustfft::num_complex::Complex;
use std::f32::consts::TAU;

pub trait WaveletFn {
    fn func(&self, t: f32) -> Complex<f32>;
}

pub struct Laplace {
    a: f32,
    b: Complex<f32>,
}

impl Laplace {
    pub fn new(zeta: f32) -> Self {
        let k = 1.0 - zeta.powi(2);
        Laplace {
            a: k.recip(),
            b: -zeta / k.sqrt() + Complex::i(),
        }
    }
}

impl WaveletFn for Laplace {
    fn func(&self, t: f32) -> Complex<f32> {
        if t >= 0.0 {
            self.a * (TAU * t * self.b).exp()
        } else {
            Complex::new(0., 0.)
        }
    }
}

pub struct Soulti {
    a: f32,
    b: f32,
}

impl Soulti {
    pub fn new(zeta: f32) -> Self {
        let k = 1.0 - zeta.powi(2);
        Soulti {
            a: k.recip(),
            b: -zeta / k.sqrt(),
        }
    }
}

impl WaveletFn for Soulti {
    fn func(&self, t: f32) -> Complex<f32> {
        if t >= 0.0 {
            Complex::from(self.a * (TAU * t * self.b).exp() * (TAU * t).sin())
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
}

impl WaveletFn for Morlet {
    fn func(&self, t: f32) -> Complex<f32> {
        (-0.5 * (t * TAU * 0.2).powi(2)).exp() * (TAU * t * Complex::i()).exp()
    }
}

// will be removed later v
pub fn soulti(t: f32, zeta: f32) -> f32 {
    let k = 1.0 - zeta.powi(2);

    if t > 0.0 {
        (-zeta / k * TAU * t).exp() * (TAU * t).sin() / k
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    // Unit step function
    fn u(t: f32) -> f32 {
        if t >= 0.0 {
            1.0
        } else {
            0.0
        }
    }

    #[test]
    fn test_laplace_wavelet() {
        for zeta in [0.1f32, 0.2f32, 0.5f32].iter() {
            let wvt = Laplace::new(*zeta);

            for t_int in 0..10 {
                let t = t_int as f32;
                let actual = wvt.func(t);

                // The unscaled wavelet function is defined for 6.28 Hz,
                // but my implementation is for 1 Hz.
                let t = t * TAU;
                let expected = 1.0 / (1.0 - zeta.powi(2))
                    * (-zeta * t / (1.0 - zeta.powi(2)).sqrt()).exp()
                    * (Complex::i() * t).exp()
                    * u(t);

                dbg!(t_int);
                assert_approx_eq!(expected.re, actual.re, 1e-6);
                assert_approx_eq!(expected.im, actual.im, 1e-6);
            }
        }
    }

    #[test]
    fn test_soulti_wavelet() {
        for zeta in [0.1f32, 0.2f32, 0.5f32].iter() {
            let wvt = Soulti::new(*zeta);

            for t_int in 0..10 {
                let t = t_int as f32;
                let actual = wvt.func(t);

                // The unscaled wavelet function is defined for 6.28 Hz,
                // but my implementation is for 1 Hz.
                let t = t * TAU;
                let expected = 1.0 / (1.0 - zeta.powi(2))
                    * (-zeta * t / (1.0 - zeta.powi(2)).sqrt()).exp()
                    * t.sin()
                    * u(t);

                dbg!(t_int);
                assert_approx_eq!(expected, actual.re, 1e-6);
                assert_approx_eq!(0.0, actual.im, 1e-6);
            }
        }
    }

    #[test]
    fn test_morlet() {}

    #[test]
    fn test_morlet_cpx() {}
}
