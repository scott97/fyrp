//! Cross correlation, discarding values so that the returned vector
//! is of equal length to the provided signal.

use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

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

// pub fn xcorr_simd(x: &[Complex<f32>], h: &[Complex<f32>]) -> Vec<f32> {}

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
}
