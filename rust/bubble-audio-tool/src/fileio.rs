use charts::{Chart, Color, MarkerType, PointLabelPosition, ScaleLinear, ScatterView};
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::env;

pub fn get_data(path: &Path) -> Option<(Vec<f32>, u32)> {
    let mut input_file = File::open(path).unwrap();
    let (header, data) = wav::read(&mut input_file).unwrap();
    let fs = header.sampling_rate;

    // Remap to range -1.0 to 1.0
    if let wav::BitDepth::Sixteen(raw_signal) = data {
        let y = raw_signal
            .iter()
            .map(|x| (*x as f32) / (i16::MAX as f32))
            .collect();
        Some((y, fs))
    } else {
        None
    }
}

// Write scaleogram data to a csv file
pub fn export_scaleogram(s: &[Vec<f32>], dir: &Path, idx: usize) {
    let name = format!("scaleogram{}.csv", idx);
    let path = dir.join(Path::new(&name));

    let mut wtr = csv::Writer::from_path(path).unwrap();
    for row in s.iter() {
        let text_vec: Vec<String> = row.iter().map(|n| format!("{:e}", n)).collect(); // Use sci notation
        wtr.write_record(&text_vec).unwrap();
    }
    wtr.flush().unwrap();
}

// Write bubble identification data to a csv file
pub fn export_bubble_data(b: &[(f32, f32)], dir: &Path, idx: usize) {
    let name = format!("bubbles{}.csv", idx);
    let path = dir.join(Path::new(&name));

    if !b.is_empty() {
        let mut wtr = csv::Writer::from_path(path).unwrap();
        let text_vec: Vec<String> = b.iter().map(|(rad, _)| format!("{:e}", rad)).collect();
        wtr.write_record(&text_vec).unwrap();
        let text_vec: Vec<String> = b.iter().map(|(_, ts)| format!("{:e}", ts)).collect();
        wtr.write_record(&text_vec).unwrap();
        wtr.flush().unwrap();
    } else {
        File::create(path).unwrap();
    }
}

// Plot bubble identification data on a graph
pub fn plot_bubble_data(b: &[(f32, f32)], dir: &Path, idx: usize) -> std::io::Result<()> {

    
        
    let name = format!("bubbles{}.svg", idx);
    let path = dir.join(Path::new(&name));

    // Define chart related sizes.
    let width = 1600;
    let height = 1200;
    let (top, right, bottom, left) = (90, 40, 50, 60); // margins?

    // Create scales
    let x = ScaleLinear::new()
        .set_domain(vec![0., 10000.])
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![0., 5.0])
        .set_range(vec![height - top - bottom, 0]);

    // Rearrange data
    let scatter_data: Vec<(f32, f32)> = b.iter().map(|(y, x)| (*x, *y)).collect();

    // Create Scatter view that is going to represent the data as points.
    let scatter_view = ScatterView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_label_visibility(false)
        .set_marker_type(MarkerType::Circle)
        .set_colors(Color::color_scheme_dark())
        .load_data(&scatter_data)
        .unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Identified Bubbles"))
        .add_view(&scatter_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Radius (mm)")
        .add_bottom_axis_label("Time (ms)")
        .save(path)
        .expect("chart could not be created");
    
    let current_dir = env::current_dir()?;

    let url = "file://".to_string() + current_dir.to_str().unwrap() + "/" + dir.to_str().unwrap() + "/" + &name;
    Command::new("C:\\Program Files\\Mozilla Firefox\\firefox.exe")
        .arg(&url)
        .output()?;

    Ok(())
}
