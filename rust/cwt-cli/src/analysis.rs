pub fn find_max(s: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    todo!();
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
