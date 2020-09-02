use rayon::prelude::*;

pub type Point = (f32, f32, f32);

struct ShiftablePoint {
    original_position: (f32, f32),
    shiftable_position: (f32, f32),
    value: f32,
}

impl PartialEq for ShiftablePoint {
    fn eq(&self, other: &Self) -> bool {
        self.shiftable_position == other.shiftable_position
    }
}

pub fn mean_shift_cluster(
    points: &Vec<Point>,
    window: fn(Point, Point) -> f32,
    max_iterations: u32,
) -> Vec<Point> {
    // The points are copied into a new list of ShiftablePoint structs, which contain the original position and a mutable position.
    let mut shifted_points: Vec<_> = points
        .into_iter()
        .map(|p| ShiftablePoint {
            original_position: (p.0, p.1),
            shiftable_position: (p.0, p.1),
            value: p.2,
        })
        .collect();

    // Each point is mean shifted, affecting only the mutable position.
    for _ in 0..max_iterations {
        shifted_points
            .par_iter_mut()
            .for_each(|mut p| shift_point(&mut p, &points, window))
    }

    // The points are split into groups, based on the shifted position.
    // The highest intensity point in each group is selected, and the original position is appended to the list of results.
    let mut result: Vec<ShiftablePoint> = Vec::new();

    for p in shifted_points {
        if !result.contains(&p) {
            result.push(p)
        } else {
            let idx = result
                .iter()
                .position(|other| *other == p && other.value < p.value);
            if let Some(i) = idx {
                result[i] = p;
            }
        }
    }

    result.into_iter().map(|p| (p.original_position.0,p.original_position.1,p.value) ).collect()
}

fn shift_point(p: &mut ShiftablePoint, points: &Vec<Point>, window: fn(Point, Point) -> f32) {
    let mut r = (0f32, 0f32); // result point, r.
    let mut weight = 0f32;
    for k in points.into_iter() {
        // other point, k.
        let w = window(
            (p.shiftable_position.0, p.shiftable_position.1, p.value),
            *k,
        );
        r.0 += w * k.0;
        r.1 += w * k.1;
        weight += w;
    }

    p.shiftable_position = (r.0 / weight, r.1 / weight);
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
            (1.10, 0.10, 1.0),
            (1.05, 0.05, 2.0),
            (4.02, 6.98, 2.0),
            (4.08, 7.03, 1.0),
            (3.95, 7.00, 1.0),
            (1.05, -0.3, 1.0),
        ];
        let mut expected = vec![(1.05, 0.05), (4.02, 6.98)];
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
