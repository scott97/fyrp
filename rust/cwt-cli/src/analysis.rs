pub fn find_bubbles(s: &Vec<Vec<f32>>) -> Vec<(usize,usize)> {
    let mut bubbles: Vec<(usize,usize)> = Vec::new();
    for row in 1..s.len()-1 {
        for col in 1..s[0].len()-1 {
            // Check it is a local maximum.
            if (s[row][col] > s[row + 1][col]
                && s[row][col] > s[row - 1][col]
                && s[row][col] > s[row][col + 1]
                && s[row][col] > s[row][col - 1])
            {
                bubbles.push((row,col));
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
