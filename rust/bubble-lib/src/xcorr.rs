//! Cross correlation, discarding values so that the returned vector
//! is of equal length to the provided signal.

use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;
use packed_simd::*;

/// A simple cross correlation implementation.
pub fn xcorr(x: &[Complex<f32>], h: &[Complex<f32>]) -> Vec<Complex<f32>> {
    let mut y = vec![Complex::zero(); x.len()];

    for i in 0..x.len() {
        for j in 0..h.len() {
            let k = i + j;
            y[i] += x.get(k).unwrap_or(&Complex::zero()) * h[j];
        }
    }

    y
}

/// A cross correlation implementation using fast fourier transforms.
/// Does not work with complex wavelets.
/// TODO: Implement for complex numbers.
pub fn xcorr_simd(x: &[Complex<f32>], h: &[Complex<f32>]) -> Vec<Complex<f32>> {
    let range = ..x.len();

    // Flatten the complex vector into two vectors of f32s.
    let mut x_re: Vec<_> = x.into_iter().map(|v| v.re).collect();
    let mut x_im: Vec<_> = x.into_iter().map(|v| v.im).collect();

    let lx = x.len();
    let lh = h.len();
    let lxch = lx - (lx % 16) + 16; // Chunked length of x, (rounded up to nearest 16).

    let mut r = vec![0.0; lxch + lh]; // Result vector.

    x_re.resize(lxch + lh, 0.); // pad right w/ zeros to chunk size.
    x_im.resize(lxch + lh, 0.); // pad right w/ zeros to chunk size.

    for m in 0..lh {
        for ch in (0..lxch).step_by(16) {
            let x_chunk = f32x16::from_slice_unaligned(&x_re[(ch + m)..(ch + m + 16)]);
            let r_chunk = f32x16::from_slice_unaligned(&r[(ch)..(ch + 16)]);
            let w_chunk = r_chunk + x_chunk * f32x16::splat(h[m].re); // chunk for writing to the result vector.
            w_chunk.write_to_slice_unaligned(&mut r[(ch)..(ch + 16)]);
        }
    }

    r[range].into_iter().map(Complex::from).collect()
}

/// A cross correlation implementation using fast fourier transforms.
pub fn xcorr_fft(
    mut sig: &mut Vec<Complex<f32>>,
    mut fir: &mut Vec<Complex<f32>>,
) -> Vec<Complex<f32>> {
    let n = sig.len() + fir.len() - 1;
    let range = (fir.len()-1)..;

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

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    fn assert_slice_approx_eq(a: &[f32], b: &[f32]) {
        println!("a: {:?}",&a);
        println!("b: {:?}",&b);

        if a.len() != b.len() {
            panic!("{:?}, {:?}", &a, &b);
        }
        for (x, y) in a.iter().zip(b.iter()) {
            if *x > *y + 1e-3 || *y > *x + 1e-3 {
                panic!("{:?}, {:?}", &a, &b);
            }
        }
    }

    fn get_test(i: isize) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
        match i {
            0 => (
                // TODO: this test case is unverified.
                vec![1.0, 2.0, 3.0],
                vec![4.0, 5.0],
                vec![14.0, 23.0, 12.0],
            ),
            1 => (
                // Verified by hand.
                vec![1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0],
                vec![1.0, 0.0],
                vec![1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            ),
            2 => (
                // Verified by hand.
                vec![1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0],
                vec![1.0, 1.0, 0.0, 0.0],
                vec![2.0, 1.0, 0.0, 1.0, 2.0, 1.0, 0.0, 0.0],
            ),
            _ => panic!("test doesn't exist"),
        }
    }

    #[test]
    fn test_xcorr_fft() {
        for i in 0..3 {
            let (x, h, expected) = get_test(i);

            let mut xc: Vec<_> = x.iter().map(Complex::from).collect();
            let mut hc: Vec<_> = h.iter().map(Complex::from).collect();

            let actual: Vec<_> = xcorr_fft(&mut xc, &mut hc)
                .iter()
                .map(|i| i.norm())
                .collect();

            assert_slice_approx_eq(&expected, &actual);
        }
    }

    #[test]
    fn test_xcorr() {
        for i in 0..3 {
            let (x, h, expected) = get_test(i);

            let xc: Vec<_> = x.iter().map(Complex::from).collect();
            let hc: Vec<_> = h.iter().map(Complex::from).collect();

            let actual: Vec<_> = xcorr(&xc, &hc).iter().map(|i| i.norm()).collect();

            assert_slice_approx_eq(&expected, &actual);
        }
    }

    #[test]
    fn test_xcorr_simd() {
        for i in 0..3 {
            let (x, h, expected) = get_test(i);

            let xc: Vec<_> = x.iter().map(Complex::from).collect();
            let hc: Vec<_> = h.iter().map(Complex::from).collect();

            let actual: Vec<_> = xcorr_simd(&xc, &hc).iter().map(|i| i.norm()).collect();

            assert_slice_approx_eq(&expected, &actual);
        }
    }
}
