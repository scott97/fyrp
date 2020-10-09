//! Cross correlation, discarding values so that the returned vector
//! is of equal length to the provided signal.

use packed_simd::*;
use rustfft::num_complex::Complex;
use rustfft::num_traits::One;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

pub mod cplx {
    use packed_simd::*;
    use rustfft::num_complex::Complex;
    use rustfft::num_traits::One;
    use rustfft::num_traits::Zero;
    use rustfft::FFTplanner;

    /// A simple cross correlation implementation.
    pub fn xcorr(x: &[Complex<f32>], h: &[Complex<f32>]) -> Vec<Complex<f32>> {
        let mut r = vec![Complex::zero(); x.len()];

        for i in 0..x.len() {
            for j in 0..h.len() {
                let k = i + j;
                r[i] += x.get(k).unwrap_or(&Complex::zero()) * h[j];
            }
        }

        r
    }

    /// A cross correlation implementation using SIMD instructions.
    pub fn xcorr_simd(x: &[Complex<f32>], h: &[Complex<f32>]) -> Vec<Complex<f32>> {
        // Lengths
        let lx = x.len();
        let lh = h.len();
        let lxch = lx - (lx % 16) + 16; // Chunked length of x, (rounded up to nearest 16).

        // Flatten the complex vector into two vectors of f32s.
        let mut x_re: Vec<_> = x.into_iter().map(|v| v.re).collect();
        let mut x_im: Vec<_> = x.into_iter().map(|v| v.im).collect();

        x_re.resize(lxch + lh, 0.); // pad right w/ zeros to chunk size.
        x_im.resize(lxch + lh, 0.); // pad right w/ zeros to chunk size.

        // Result vectors.
        let mut r_re = vec![0.0; lxch + lh];
        let mut r_im = vec![0.0; lxch + lh];

        for m in 0..lh {
            for ch in (0..lxch).step_by(16) {
                let x_re_chunk = f32x16::from_slice_unaligned(&x_re[(ch + m)..(ch + m + 16)]);
                let x_im_chunk = f32x16::from_slice_unaligned(&x_im[(ch + m)..(ch + m + 16)]);
                let r_re_chunk = f32x16::from_slice_unaligned(&r_re[(ch)..(ch + 16)]);
                let r_im_chunk = f32x16::from_slice_unaligned(&r_im[(ch)..(ch + 16)]);

                let h_re_chunk = f32x16::splat(h[m].re);
                let h_im_chunk = f32x16::splat(h[m].im);

                // chunk for writing to the result vectors.
                let w_re_chunk = r_re_chunk + x_re_chunk * h_re_chunk - x_im_chunk * h_im_chunk;
                let w_im_chunk = r_im_chunk + x_re_chunk * h_im_chunk + x_im_chunk * h_re_chunk;

                w_re_chunk.write_to_slice_unaligned(&mut r_re[(ch)..(ch + 16)]);
                w_im_chunk.write_to_slice_unaligned(&mut r_im[(ch)..(ch + 16)]);
            }
        }

        // Combine the two vectors into one
        r_re[..x.len()]
            .iter()
            .zip(r_im.iter())
            .map(|(re, im)| Complex::new(*re, *im))
            .collect()
    }

    /// A cross correlation implementation using fast fourier transforms.
    pub fn xcorr_fft(
        mut sig: &mut Vec<Complex<f32>>,
        mut fir: &mut Vec<Complex<f32>>,
    ) -> Vec<Complex<f32>> {
        let n = sig.len() + fir.len() - 1;
        let range = (fir.len() - 1)..;

        // Time reverse and resize the fir filter.
        fir.reverse();
        fir.resize(n, Complex::zero());

        // Resize the signal.
        sig.resize(n, Complex::zero());
        // Frequency domain
        let mut fsig: Vec<Complex<f32>> = vec![Complex::zero(); n];
        let mut ffir: Vec<Complex<f32>> = vec![Complex::zero(); n];

        // Do FFT
        let fft = FFTplanner::new(false).plan_fft(n);
        fft.process(&mut sig, &mut fsig);
        fft.process(&mut fir, &mut ffir);

        // Elementwise multiplication
        // Dividing each individually by sqrt(n) is the same as dividing both by n.
        let mut fres: Vec<Complex<f32>> = vec![Complex::zero(); n];
        let n_inv = 1. / (n as f32);
        for i in 0..n {
            fres[i] = fsig[i] * ffir[i] * n_inv;
        }

        // Do IFFT
        let mut result: Vec<Complex<f32>> = vec![Complex::zero(); n];
        let ifft = FFTplanner::new(true).plan_fft(n);
        ifft.process(&mut fres, &mut result);

        result[range].to_vec()
    }
}

/// Cross correlation optimised for real numbers only
pub mod real {
    use packed_simd::*;
    use rustfft::num_complex::Complex;
    use rustfft::num_traits::One;
    use rustfft::num_traits::Zero;
    use rustfft::FFTplanner;

    pub fn xcorr(x: &[f32], h: &[f32]) -> Vec<f32> {
        let mut r = vec![0.0; x.len()];
        for i in 0..x.len() {
            for j in 0..h.len() {
                let k = i + j;
                r[i] += x.get(k).unwrap_or(&0.0) * h[j];
            }
        }
        r
    }

    /// A cross correlation implementation using SIMD instructions.
    /// TODO: read out of range error.
    pub fn xcorr_simd(x: &[f32], h: &[f32]) -> Vec<f32> {
        // Lengths
        let lx = x.len();
        let lh = h.len();
        let lxch = lx - (lx % 16) + 16; // Chunked length of x, (rounded up to nearest 16).

        // Pad right w/ zeros to chunk size.
        x.to_vec().resize(lxch + lh, 0.);

        // Result vector.
        let mut r = vec![0.0; lxch + lh];

        for m in 0..lh {
            for ch in (0..lxch).step_by(16) {
                let x_chunk = f32x16::from_slice_unaligned(&x[(ch + m)..(ch + m + 16)]);
                let r_chunk = f32x16::from_slice_unaligned(&r[(ch)..(ch + 16)]);
                let h_chunk = f32x16::splat(h[m]);

                // chunk for writing to the result vectors.
                let w_chunk = r_chunk + x_chunk * h_chunk;

                w_chunk.write_to_slice_unaligned(&mut r[(ch)..(ch + 16)]);
            }
        }

        // Combine the two vectors into one
        r[..x.len()].to_vec()
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_slice_approx_eq(a: &[Complex<f32>], b: &[Complex<f32>]) {
        println!("a: {:?}", &a);
        println!("b: {:?}", &b);

        if a.len() != b.len() {
            panic!("{:?}, {:?}", &a, &b);
        }
        for (x, y) in a.iter().zip(b.iter()) {
            if x.re > y.re + 1e-3 || y.re > x.re + 1e-3 {
                panic!("{:?}, {:?}", &a, &b);
            }
            if x.im > y.im + 1e-3 || y.im > x.im + 1e-3 {
                panic!("{:?}, {:?}", &a, &b);
            }
        }
    }

    fn get_test(i: isize) -> (Vec<Complex<f32>>, Vec<Complex<f32>>, Vec<Complex<f32>>) {
        match i {
            0 => (
                // TODO: this test case is unverified.
                vec![Complex::from(1.0), Complex::from(2.0), Complex::from(3.0)],
                vec![Complex::from(4.0), Complex::from(5.0)],
                vec![
                    Complex::from(14.0),
                    Complex::from(23.0),
                    Complex::from(12.0),
                ],
            ),
            1 => (
                // Verified by hand.
                vec![
                    Complex::from(1.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::from(-1.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                ],
                vec![Complex::from(1.0), Complex::zero()],
                vec![
                    Complex::from(1.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::from(-1.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                ],
            ),
            2 => (
                // Verified by hand.
                vec![
                    Complex::from(1.0),
                    Complex::from(1.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::from(1.0),
                    Complex::from(1.0),
                    Complex::zero(),
                    Complex::zero(),
                ],
                vec![
                    Complex::from(1.0),
                    Complex::from(1.0),
                    Complex::zero(),
                    Complex::zero(),
                ],
                vec![
                    Complex::from(2.0),
                    Complex::from(1.0),
                    Complex::zero(),
                    Complex::from(1.0),
                    Complex::from(2.0),
                    Complex::from(1.0),
                    Complex::zero(),
                    Complex::zero(),
                ],
            ),
            3 => (
                // Verified by hand.
                vec![
                    Complex::from(1.0),
                    Complex::from(1.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::from(-1.0),
                    Complex::from(1.0),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::i(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::from(4.0),
                    Complex::zero(),
                    Complex::new(6.0, 6.0),
                    Complex::from(3.0),
                    Complex::zero(),
                ],
                vec![
                    Complex::i(),
                    Complex::zero(),
                    Complex::one(),
                    Complex::zero(),
                ],
                vec![
                    Complex::i(),
                    Complex::i(),
                    -Complex::one(),
                    Complex::one(),
                    -Complex::i(),
                    Complex::i(),
                    Complex::i(),
                    Complex::zero(),
                    -Complex::one(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::zero(),
                    Complex::from(4.0),
                    Complex::zero(),
                    Complex::new(6.0, 10.0),
                    Complex::from(3.0),
                    Complex::new(-6.0, 6.0),
                    Complex::new(0.0, 3.0),
                    Complex::zero(),
                ],
            ),
            _ => panic!("test doesn't exist"),
        }
    }

    #[test]
    fn test_xcorr_fft() {
        for i in 0..4 {
            let (mut x, mut h, expected) = get_test(i);

            let actual = xcorr_fft(&mut x, &mut h);

            assert_slice_approx_eq(&expected, &actual);
        }
    }

    #[test]
    fn test_xcorr() {
        for i in 0..4 {
            let (x, h, expected) = get_test(i);

            let actual = xcorr(&x, &h);

            assert_slice_approx_eq(&expected, &actual);
        }
    }

    #[test]
    fn test_xcorr_simd() {
        for i in 0..4 {
            let (x, h, expected) = get_test(i);

            let actual = xcorr_simd(&x, &h);

            assert_slice_approx_eq(&expected, &actual);
        }
    }
}
