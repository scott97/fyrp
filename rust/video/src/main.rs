// Installation guide
// https://github.com/twistedfall/opencv-rust/issues/118#issuecomment-619608278

use opencv::{
    core, highgui,
    prelude::{BackgroundSubtractorMOG2, MatTraitManual,VideoCaptureTrait},
    video,
    videoio::{VideoCapture,CAP_ANY},
    imgproc::*,
};


// Parameters to adjust
const HISTORY: i32 = 200;
const MOG_THRESHOLD: f64 = 50.0;
const KERNEL_SIZE: i32 = 4;
const MORPH_ITERATIONS: i32 = 8;
const FILE: &str = "1.mp4";


fn run() -> opencv::Result<()> {
    let video_window = "video capture";
    highgui::named_window(video_window, 1)?;

    let cv_window = "mog_result";
    highgui::named_window(cv_window, 1)?;

    let mut mog2 = video::create_background_subtractor_mog2(HISTORY, MOG_THRESHOLD, false)?;

    let mut cap = VideoCapture::from_file(FILE, CAP_ANY)?;

    let kernel = get_structuring_element(
        MorphShapes::MORPH_ELLIPSE as i32, 
        core::Size_::new(KERNEL_SIZE, KERNEL_SIZE), 
        core::Point_::new(0,0)
    )?;

    loop {
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
            core::Point_::new(0,0),
            MORPH_ITERATIONS, 
            core::BorderTypes::BORDER_CONSTANT as i32, 
            morphology_default_border_value()?
        )?;

        if frame.size()?.width > 0 {
            highgui::imshow(video_window, &frame)?;
        }
        if frame_mog_result.size()?.width > 0 {
            highgui::imshow(cv_window, &frame_morph_result)?;
        }

        let key = highgui::wait_key(10)?;
        if key > 0 && key != 255 {
            break;
        }
    }
    Ok(())
}

fn main() {
    run().unwrap()
}
