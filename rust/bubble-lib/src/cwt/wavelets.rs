use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use std::f32::consts::TAU;

pub trait WaveletFn {
    fn real(&self, t: f32) -> f32;
    fn cplx(&self, t: f32) -> Complex<f32>;
}

pub struct Laplace {
    a: f32,
    b: f32,
}

impl Laplace {
    pub fn new(zeta: f32) -> Self {
        let k = 1.0 - zeta.powi(2);
        Laplace {
            a: k.recip(),
            b: -zeta / k.sqrt(),
        }
    }
}

impl WaveletFn for Laplace {
    // The complex version of this wavelet is called the Laplace wavelet
    fn cplx(&self, t: f32) -> Complex<f32> {
        if t >= 0.0 {
            self.a * (TAU * t * self.b).exp() * (TAU * t * Complex::i()).exp()
        } else {
            Complex::zero()
        }
    }

    // The real version of this wavelet is called the SOULTI wavelet
    fn real(&self, t: f32) -> f32 {
        if t >= 0.0 {
            self.a * (TAU * t * self.b).exp() * (TAU * t).sin()
        } else {
            0.0
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
    fn real(&self, t: f32) -> f32 {
        (-0.5 * (t * TAU * 0.2).powi(2)).exp() * (TAU * t).cos()
    }
    fn cplx(&self, t: f32) -> Complex<f32> {
        (-0.5 * (t * TAU * 0.2).powi(2)).exp() * (TAU * t * Complex::i()).exp()
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

            for t_int in -5..5 {
                let t = t_int as f32;
                let actual = wvt.cplx(t);

                // The unscaled wavelet function is defined for 6.28 Hz,
                // but my implementation is for 1 Hz.
                let t = t * TAU;

                // Reference formula.
                // https://www.researchgate.net/publication/316869945_Laplace_wavelet_transform_theory_and_applications
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
            let wvt = Laplace::new(*zeta);

            for t_int in -5..5 {
                let t = t_int as f32;
                let actual = wvt.real(t);

                // The unscaled wavelet function is defined for 6.28 Hz,
                // but my implementation is for 1 Hz.
                let t = t * TAU;

                // Reference formula.
                // https://www.researchgate.net/publication/309287880_A_new_wavelet_family_based_on_second-order_LTI-systems
                let expected = 1.0 / (1.0 - zeta.powi(2))
                    * (-zeta * t / (1.0 - zeta.powi(2)).sqrt()).exp()
                    * t.sin()
                    * u(t);

                dbg!(t_int);
                assert_approx_eq!(expected, actual, 1e-6);
            }
        }
    }

    #[test]
    fn test_morlet() {
        let wvt = Morlet::new();

        for t_int in -5..5 {
            let t = t_int as f32;
            let actual = wvt.real(t);

            // The unscaled wavelet function is defined for 6.28/5 Hz,
            // but my implementation is for 1 Hz.
            let t = t * TAU / 5.0;

            // Reference formula.
            // https://towardsdatascience.com/what-is-wavelet-and-how-we-use-it-for-data-science-d19427699cef#7c08
            let expected = (-t.powi(2) / 2.0).exp() * (5.0 * t).cos();
            dbg!(t_int);
            assert_approx_eq!(expected, actual, 1e-6);
        }
    }

    #[test]
    fn test_morlet_cpx() {
        let wvt = Morlet::new();

        for t_int in -5..5 {
            let t = t_int as f32;
            let actual = wvt.cplx(t);

            // The unscaled wavelet function is defined for 6.28/5 Hz,
            // but my implementation is for 1 Hz.
            let t = t * TAU / 5.0;

            // Reference formula.
            // TODO: find somewhere which verifies this.
            let expected = (-t.powi(2) / 2.0).exp() * (5.0 * t * Complex::i()).exp();

            dbg!(t_int);
            assert_approx_eq!(expected.re, actual.re, 1e-6);
            assert_approx_eq!(expected.im, actual.im, 1e-6);
        }
    }
}
