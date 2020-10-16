// Installation guide
// https://github.com/twistedfall/opencv-rust/issues/118#issuecomment-619608278

use opencv::{
    core, highgui,
    imgproc::*,
    prelude::{BackgroundSubtractorMOG2, MatTraitManual, VideoCaptureTrait},
    types, video,
    videoio::{VideoCapture, CAP_ANY},
};

// Parameters to adjust
const HISTORY: i32 = 200;
const MOG_THRESHOLD: f64 = 50.0;
const KERNEL_SIZE: i32 = 4;
const MORPH_ITERATIONS: i32 = 8;
const FILE: &str = "1.mp4";
const MM_PER_PIXEL: f32 = 100.0 / 458.0; // 100 mm = 458 pixels
const RECT: [(i32, i32); 2] = [(850, 950), (1050, 1025)]; // [ (x1,y1), (x2,y2) ]
const SLOW_MODE: bool = false;
const FPS: i32 = 30; // 30 FPS. I think when I trimmed it in windows 10 video editor it reduced its framerate.
const DURATION: i32 = 2 * 60;


#[derive(Debug)]
struct Bubble {
    radius: f32,
    time: f32,
}

fn run() -> opencv::Result<()> {
    let mut results: Vec<Bubble> = Vec::new();

    let video_window = "video capture";
    highgui::named_window(video_window, highgui::WINDOW_NORMAL)?; // video is 1920x1080
    highgui::resize_window(video_window, 960, 540)?; // resize x1/2

    let cv_window = "mog_result";
    highgui::named_window(cv_window, highgui::WINDOW_NORMAL)?; // video is 1920x1080
    highgui::resize_window(cv_window, 960, 540)?; // resize x1/2

    let mut mog2 = video::create_background_subtractor_mog2(HISTORY, MOG_THRESHOLD, false)?;

    let mut cap = VideoCapture::from_file(FILE, CAP_ANY)?;

    let kernel = get_structuring_element(
        MorphShapes::MORPH_ELLIPSE as i32,
        core::Size_::new(KERNEL_SIZE, KERNEL_SIZE),
        core::Point_::new(0, 0),
    )?;

    let rect = core::Rect_::from_points(
        core::Point_::new(RECT[0].0, RECT[0].1),
        core::Point_::new(RECT[1].0, RECT[1].1),
    );

    let rect_f = core::Rect_::from_points(
        core::Point_::new(RECT[0].0 as f32, RECT[0].1 as f32),
        core::Point_::new(RECT[1].0 as f32, RECT[1].1 as f32),
    );

    let mut rect_occupied_previous = false;

    for frame_num in 1..(FPS*DURATION) {
        let mut frame = core::Mat::default()?;
        let mut frame_mog_result = core::Mat::default()?;
        let mut frame_morph_result = core::Mat::default()?;
        
        cap.read(&mut frame)?;
    
        mog2.apply(&frame, &mut frame_mog_result, -1.0)?;
        morphology_ex(
            &frame_mog_result,
            &mut frame_morph_result,
            MORPH_CLOSE,
            &kernel,
            core::Point_::new(0, 0),
            MORPH_ITERATIONS,
            core::BorderTypes::BORDER_CONSTANT as i32,
            morphology_default_border_value()?,
        )?;

        let mut contours: types::VectorOfVectorOfPoint = core::Vector::new();
        find_contours(
            &frame_morph_result,
            &mut contours,
            RetrievalModes::RETR_TREE as i32,
            ContourApproximationModes::CHAIN_APPROX_SIMPLE as i32,
            core::Point_::new(0, 0),
        )?;

        let mut rect_occupied = false;
        for i in 0..contours.len() {
            let mut centre: core::Point2f = core::Point_::new(0.0, 0.0);
            let mut radius = 0.0;

            min_enclosing_circle(&contours.get(i)?, &mut centre, &mut radius)?;

            // if centre is within rectange
            if centre.inside(rect_f) {
                rect_occupied = true;

                if !rect_occupied_previous {
                    // bubble has entered the rectangle
                    results.push(Bubble{radius: radius * MM_PER_PIXEL, time: frame_num as f32 / FPS as f32})
                }
            }
        }
        rect_occupied_previous = rect_occupied;

        rectangle(
            &mut frame,
            rect,
            core::Scalar_::new(0.0, 0.0, 255.0, 255.0),
            2,
            LINE_8,
            0,
        )?;

        if frame.size()?.width > 0 {
            highgui::imshow(video_window, &frame)?;
        }
        if frame_mog_result.size()?.width > 0 {
            highgui::imshow(cv_window, &frame_morph_result)?;
        }

        let wait = if SLOW_MODE { 500 } else { 1 };
        let key = highgui::wait_key(wait)?;
        if key > 0 && key != 255 {
            break;
        }
    }


    println!("{:?}",results);
    Ok(())
}

fn main() {
    run().unwrap()
}
