use itertools::Itertools;
use packed_simd::*;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

// do convolution the normal way
pub fn conv(x: &Vec<f32>, h: &Vec<f32>) -> Vec<f32> {
    let n = x.len() + h.len() - 1;
    let mut y: Vec<f32> = vec![0.0; n];

    for i in 0..n {
        for j in 0..h.len() {
            if let Some(d) = i.checked_sub(j) {
                y[i] += x.get(d).unwrap_or(&0.0) * h[j];
            }
        }
    }

    y
}

// Very nearly works except for the last few elements which are sometimes left as zeros
pub fn conv_simd(x: &Vec<f32>, h: &Vec<f32>) -> Vec<f32> {
    let lx = x.len();
    let lh = h.len();
    let lxch = lx - (lx % 16) + 16; // Chunked length of x, (rounded up to nearest 16).

    let mut y = vec![0.0; lxch + lh];

    let mut xm = vec![0.; lh];
    xm.extend(x.iter()); // pad left w/ zeros so shifts of x don't read outside array.
    xm.resize(lxch + lh, 0.); // pad right w/ zeros to chunk size.

    for m in 0..lh {
        for ch in (0..lxch).step_by(16) {
            let x_chunk = f32x16::from_slice_unaligned(&xm[(ch + lh - m)..(ch + lh - m + 16)]);
            let y_chunk = f32x16::from_slice_unaligned(&y[(ch)..(ch + 16)]);
            let result = y_chunk + x_chunk * f32x16::splat(h[m]);
            result.write_to_slice_unaligned(&mut y[(ch)..(ch + 16)]);
        }
    }

    y.truncate(lx + lh - 1);
    y
}

// do convolution using FFT
pub fn conv_fft(sig: &Vec<f32>, fir: &Vec<f32>) -> Vec<f32> {
    let n = sig.len() + fir.len() - 1;

    // Time domain
    let mut tsig: Vec<Complex<f32>> = sig
        .iter()
        .pad_using(n, |_i| &0.0)
        .map(|t| Complex::from(t))
        .collect();
    let mut tfir: Vec<Complex<f32>> = fir
        .iter()
        .pad_using(n, |_i| &0.0)
        .map(|t| Complex::from(t))
        .collect();

    // Frequency domain
    let mut fsig: Vec<Complex<f32>> = vec![Complex::zero(); n];
    let mut ffir: Vec<Complex<f32>> = vec![Complex::zero(); n];

    // Do FFT
    let fft = FFTplanner::new(false).plan_fft(n);
    fft.process(&mut tsig, &mut fsig);
    fft.process(&mut tfir, &mut ffir);

    // Elementwise multiplication
    // Dividing each individually by sqrt(n) is the same as dividing both by n.
    let mut fres: Vec<Complex<f32>> = vec![Complex::zero(); n];
    let n_inv = 1. / (n as f32);
    for i in 0..n {
        fres[i] = fsig[i] * ffir[i] * n_inv;
    }

    // Do IFFT
    let mut tres: Vec<Complex<f32>> = vec![Complex::zero(); n];
    let fft = FFTplanner::new(true).plan_fft(n);
    fft.process(&mut fres, &mut tres);

    // Make real and return
    let result: Vec<f32> = tres.iter().map(|i| i.re).collect();
    result
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv() {
        let x = vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
        let h = vec![-3., 0., 3.];
        let expected = vec![-3., -6., -6., -6., -6., -6., -6., -6., -6., -6., 27., 30.];

        assert_eq!(conv(&x, &h), expected);
    }

    #[test]
    fn test_conv_fft() {
        let x = vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
        let h = vec![-3., 0., 3.];
        let expected = vec![-3., -6., -6., -6., -6., -6., -6., -6., -6., -6., 27., 30.];

        let actual = conv_fft(&x, &h);
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_relative_eq!(expected[i], actual[i], max_relative = 0.00001);
        }
    }

    #[test]
    fn test_conv_simd() {
        let x: Vec<f32> = (1..20).map(|n| n as f32).collect();
        let h = vec![-3., 0., 3.];
        let expected = vec![
            -3., -6., -6., -6., -6., -6., -6., -6., -6., -6., -6., -6., -6., -6., -6., -6., -6.,
            -6., -6., 54., 57.,
        ];

        assert_eq!(conv_simd(&x, &h), expected);
    }

    #[test]
    fn test_conv_simd_2() {
        // A second example
        let x: Vec<f32> = (1..30).map(|n| n as f32).collect();
        let h = vec![4., 4., 0., 0., 2., 2.];
        let expected = vec![
            4., 12., 20., 28., 38., 50., 62., 74., 86., 98., 110., 122., 134., 146., 158., 170.,
            182., 194., 206., 218., 230., 242., 254., 266., 278., 290., 302., 314., 326., 218.,
            106., 110., 114., 58.,
        ];

        assert_eq!(conv_simd(&x, &h), expected);
    }
}
