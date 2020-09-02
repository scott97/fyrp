use std::f32::consts::TAU;
use crate::mean_shift_clustering::mean_shift_cluster;
use crate::mean_shift_clustering::ellipse_window;

pub fn find_bubbles(s: &Vec<Vec<f32>>, frequencies: &Vec<f32>, fs: u32) -> Vec<(f32, f32)> {
    let mut peaks: Vec<(f32, f32)> = Vec::new();
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
                peaks.push((freq, time));
            }
        }
    }

    mean_shift_cluster(&peaks, |a,b| ellipse_window(a, b, (15.,5.)), 20).into_iter().map(|(f,t)| (to_radius(f),t)).collect()
}

pub fn threshold(s: &mut Vec<Vec<f32>>, min: f32) {
    for row in s.into_iter() {
        for val in row {
            if *val < min {
                *val = 0.;
            }
        }
    }
}

fn to_radius(freq: f32) -> f32 {
    (3f32 * 1.4f32 * 101.325f32).sqrt() / (freq * TAU)
}
