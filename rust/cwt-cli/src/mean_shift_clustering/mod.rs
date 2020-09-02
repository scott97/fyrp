use rayon::prelude::*;

type Point = (f32, f32);

pub fn mean_shift_cluster(
    points: &Vec<Point>,
    window: fn(Point, Point) -> f32,
    max_iterations: u32,
) -> Vec<Point> {
    let mut shifted_points = points.to_vec();
    for _ in 0..max_iterations {
        shifted_points = shifted_points
            .into_par_iter()
            .map(|p| shift_point(p, &points, window))
            .collect();
    }
    remove_duplicates(&shifted_points)
}

fn remove_duplicates(points: &Vec<Point>) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for p in points {
        if !result.contains(p) {
            result.push(*p)
        }
    }
    result
}

fn shift_point(p: Point, points: &Vec<Point>, window: fn(Point, Point) -> f32) -> Point {
    let mut r = (0f32, 0f32); // result point, r.
    let mut weight = 0f32;
    for k in points.into_iter() {
        // other point, k.
        let w = window(p, *k);
        r.0 += w * k.0;
        r.1 += w * k.1;
        weight += w;
    }

    (r.0 / weight, r.1 / weight)
}

pub fn circular_window(a: Point, b: Point, radius: f32) -> f32 {
    let delta = (b.0 - a.0, b.1 - a.1);
    if delta.0 > radius || delta.1 > radius {
        0f32 // Early return optimisation.
    } else if delta.0.powi(2) + delta.1.powi(2) > radius.powi(2) {
        0f32
    } else {
        1f32
    }
}

pub fn ellipse_window(a: Point, b: Point, axis: (f32, f32)) -> f32 {
    let delta = (b.0 - a.0, b.1 - a.1);
    if delta.0 > axis.0 || delta.1 > axis.1 {
        0f32 // Early return optimisation.
    } else if (delta.0 / axis.0).powi(2) + (delta.1 / axis.1).powi(2) > 1. {
        0f32
    } else {
        1f32
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clustering_1_iteration() {
        // This example has points very close to each other. As a result, it only takes one iteration to converge.
        let points = vec![
            (1.10, 0.10),
            (1.05, 0.05),
            (4.02, 6.98),
            (4.08, 7.03),
            (3.95, 7.00),
            (1.05, -0.3),
        ];
        let mut expected = vec![(1.0666667, -0.05), (4.016667, 7.0033336)];
        let mut result = mean_shift_cluster(&points, |a, b| circular_window(a, b, 1.5), 1);

        result.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap()
                .then(a.1.partial_cmp(&b.1).unwrap())
        });
        expected.sort_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .unwrap()
                .then(a.1.partial_cmp(&b.1).unwrap())
        });

        assert_eq!(expected, result)
    }
}
