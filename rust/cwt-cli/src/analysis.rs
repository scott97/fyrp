use std::f32::consts::TAU;

pub fn find_bubbles(s: &Vec<Vec<f32>>, frequencies: &Vec<f32>) -> Vec<(f32,usize)> {
    let mut bubbles: Vec<(f32,usize)> = Vec::new();
    for row in 1..s.len()-1 {
        for col in 1..s[0].len()-1 {
            // Check it is a local maximum.
            if (s[row][col] > s[row + 1][col]
                && s[row][col] > s[row - 1][col]
                && s[row][col] > s[row][col + 1]
                && s[row][col] > s[row][col - 1])
            {
                let f = frequencies[row];
                let radius = to_radius(f);
                bubbles.push((radius,col));
            }
        }
    }

    bubbles
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
    (3f32*1.4f32*101.325f32).sqrt() / (freq*TAU)
}