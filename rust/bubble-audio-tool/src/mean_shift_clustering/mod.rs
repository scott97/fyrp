use rayon::prelude::*;
use crate::config;

#[derive(Debug)]
pub struct Point {
    pub position: (f32, f32),
    pub value: f32,
}

impl Point {
    fn from_shiftable_point(p: &ShiftablePoint) -> Point {
        Point {
            position: p.position,
            value: p.value,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.value == other.value
    }
}

struct ShiftablePoint {
    position: (f32, f32),
    group: (f32, f32),
    value: f32,
}

impl ShiftablePoint {
    fn from_point(p: &Point) -> ShiftablePoint {
        ShiftablePoint {
            position: p.position,
            group: p.position,
            value: p.value,
        }
    }
}

impl PartialEq for ShiftablePoint {
    fn eq(&self, other: &Self) -> bool {
        self.group == other.group
    }
}

pub struct MeanShiftClustering {
    window: Box<dyn WindowFn>,
    max_iterations: u32,
}

impl MeanShiftClustering {
    pub fn new(opt: &config::Opt) -> Self {
        let bw = opt.clustering_window_bandwidths.to_vec();

        match opt.clustering_window {
            config::ClusteringWindow::Circular => assert_eq!(bw.len(),1),
            _ => assert_eq!(bw.len(),2),
        };

        let window: Box<dyn WindowFn> = match opt.clustering_window {
            config::ClusteringWindow::Circular => box move|a, b| circular_window(a, b, bw[0]),
            _ => box move|a, b| ellipse_window(a, b, (bw[0], bw[1])),
        };

        let max_iterations = opt.max_iterations;

        MeanShiftClustering {
            window,
            max_iterations,
        }
    }

    pub fn cluster(&self, points: &[Point]) -> Vec<Point> {
        // The points are copied into a new list of ShiftablePoint structs, which contain the original position and a mutable position.
        let mut shifted_points: Vec<_> = points
            .iter()
            .map(|p| ShiftablePoint::from_point(p))
            .collect();

        // Each point is mean shifted, affecting only the mutable position.
        for _ in 0..self.max_iterations {
            shifted_points
                .par_iter_mut()
                .for_each(|mut p| self.shift_point(&mut p, &points))
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

        result
            .into_iter()
            .map(|p| Point::from_shiftable_point(&p))
            .collect()
    }

    fn shift_point(&self, 
        p: &mut ShiftablePoint,
        points: &[Point],
    ) {
        let mut r = (0f32, 0f32); // result position, r.
        let mut weight = 0f32;
        for k in points.iter() {
            // other point, k.
            let w = (self.window)(p.group, k.position);
            r.0 += w * k.position.0;
            r.1 += w * k.position.1;
            weight += w;
        }
    
        p.group = (r.0 / weight, r.1 / weight);
    }
}

trait WindowFn = Send + Sync + Fn((f32, f32), (f32, f32)) -> f32;

#[allow(clippy::if_same_then_else)]
pub fn circular_window(a: (f32, f32), b: (f32, f32), radius: f32) -> f32 {
    let delta = (b.0 - a.0, b.1 - a.1);
    if delta.0 > radius || delta.1 > radius {
        0f32 // Early return optimisation.
    } else if delta.0.powi(2) + delta.1.powi(2) > radius.powi(2) {
        0f32
    } else {
        1f32
    }
}

#[allow(clippy::if_same_then_else)]
pub fn ellipse_window(a: (f32, f32), b: (f32, f32), axis: (f32, f32)) -> f32 {
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
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_circular_window() {
//         let centre = (0., 0.);
//         let radius = 5.0;

//         #[rustfmt::skip]
//         let points = vec![
//             (0.00, 2.00),  (0.00, 4.99),  (2.00, 0.00),  (4.99, 0.00),   // Testing single numbers less than the radius.
//             (0.00, -2.00), (0.00, -4.99), (-2.00, 0.00), (-4.99, 0.00),  // Testing negative numbers.
//             (2.99, 3.99),  (-2.99, 3.99), (2.99, -3.99), (-2.99, -3.99), // Testing pairs less than the radius.
//             (0.00, 6.00),  (0.00, 9.00),  (6.00, 0.00),  (9.00, 0.00),   // Testing single numbers greater than the radius.
//             (0.00, -6.00), (0.00, -9.00), (-6.00, 0.00), (-9.00, 0.00),  // Testing negative numbers.
//             (4.00, 4.00),  (-4.00, 4.00), (4.00, -4.00), (-4.00, -4.00), // Testing pairs greater than the radius.
//         ];

//         let expected = vec![
//             1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., // Inside the circle.
//             0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., 0., // Outside the circle.
//         ];

//         for i in 0..points.len() {
//             let actual = circular_window(centre, points[i], radius);
//             assert_eq!(expected[i], actual);
//         }
//     }

//     #[test]
//     fn test_clustering_1_iteration() {
//         #[rustfmt::skip]
//         let points = vec![
//             Point{ position: (1.10, 0.10), value: 1.0 },
//             Point{ position: (1.05, 0.05), value: 2.0 },
//             Point{ position: (4.02, 6.98), value: 2.0 },
//             Point{ position: (4.08, 7.03), value: 1.0 },
//             Point{ position: (3.95, 7.00), value: 1.0 },
//             Point{ position: (1.05, -0.3), value: 1.0 },
//         ];

//         #[rustfmt::skip]
//         let mut expected = vec![
//             Point{ position: (1.05, 0.05), value: 2.0 },
//             Point{ position: (4.02, 6.98), value: 2.0 },
//         ];

//         let mut actual = mean_shift_cluster(&points, |a, b| circular_window(a, b, 1.5), 1);

//         // Sort vectors so that they can be compared.
//         actual.sort_by(|a, b| {
//             a.position
//                 .0
//                 .partial_cmp(&b.position.0)
//                 .unwrap()
//                 .then(a.position.1.partial_cmp(&b.position.1).unwrap())
//         });
//         expected.sort_by(|a, b| {
//             a.position
//                 .0
//                 .partial_cmp(&b.position.0)
//                 .unwrap()
//                 .then(a.position.1.partial_cmp(&b.position.1).unwrap())
//         });

//         assert_eq!(expected, actual);
//     }
// }
