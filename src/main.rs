use opencv::{highgui, prelude::*, videoio, Result};

fn main() -> Result<()> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?;
    let mut frame = Mat::default();
    
    // moving wait_key(1) in the loop header allows for easy loop breaking if a condition is met (0 corresponds to 'ESC', 113 would be 'Q'
    while highgui::wait_key(1)? < 0 {
        cam.read(&mut frame)?;

        // check whether VideoCapture still has frames to capture
        if !cam.grab()? {
            println!("Video processing finished");
            break
        }

        highgui::imshow("window", &frame)?;
    }

    Ok(())
}
