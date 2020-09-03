use crate::mean_shift_clustering::ellipse_window;
use crate::mean_shift_clustering::mean_shift_cluster;
use crate::mean_shift_clustering::Point;
use std::f32::consts::TAU;

pub fn find_bubbles(s: &[Vec<f32>], frequencies: &[f32], fs: u32) -> Vec<(f32, f32)> {
    let mut peaks: Vec<Point> = Vec::new();
    for row in 1..s.len() - 1 {
        for col in 1..s[0].len() - 1 {
            // Check it is a local maximum.
            if s[row][col] > s[row + 1][col]
                && s[row][col] > s[row - 1][col]
                && s[row][col] > s[row][col + 1]
                && s[row][col] > s[row][col - 1]
            {
                let freq = frequencies[row] * 1e-3; // kHz
                let time = (col as f32) / (fs as f32) * 1e3; // ms
                let value = s[row][col];
                // let p = (freq, time, value);
                let p = Point {
                    position: (freq, time),
                    value,
                };
                peaks.push(p);
            }
        }
    }

    mean_shift_cluster(&peaks, |a, b| ellipse_window(a, b, (15., 15.)), 20)
        .into_iter()
        .map(|p| (to_radius(p.position.0), p.position.1))
        .collect()
}

pub fn threshold(s: &mut Vec<Vec<f32>>, min: f32) {
    for row in s.iter_mut() {
        for val in row {
            if *val < min {
                *val = 0.;
            }
        }
    }
}

pub fn to_radius(freq: f32) -> f32 {
    (3f32 * 1.4f32 * 101.325f32).sqrt() / (freq * TAU)
}

pub fn to_freq(radius: f32) -> f32 {
    (3f32 * 1.4f32 * 101.325f32).sqrt() / (radius * TAU)
}
