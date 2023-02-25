use opencv::{
    core::{Mat, Vector},
    prelude::*,
    videoio::{self, CaptureTrait, VideoCapture},
};
use std::{thread, time};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start a TCP listener to receive the frames
    let mut listener = TcpListener::bind("0.0.0.0:8080")?;
    
    // Open the video capture device
    let mut cap = VideoCapture::new("/dev/video0", videoio::CAP_ANY)?;
    cap.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
    cap.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
    let fps = cap.get(videoio::CAP_PROP_FPS)?;
    let delay = time::Duration::from_millis((1000.0 / fps) as u64);
    
    // Start capturing and processing frames
    loop {
        // Capture a frame from the video source
        let mut frame = Mat::default()?;
        cap.read(&mut frame)?;
        
        // Convert the frame to a byte buffer
        let mut buffer = vec![];
        imencode(".jpg", &frame, &mut buffer, &Vector::new())?;
        
        // Send the buffer over the network
        tokio::task::spawn(async move {
            let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
            stream.write_all(&buffer).await.unwrap();
        });
        
        // Sleep for a short period to limit the FPS
        thread::sleep(delay);
    }
}

#[tokio::main]
async fn receiver() -> Result<(), Box<dyn std::error::Error>> {
    // Accept incoming connections and receive the frames
    loop {
        let (mut stream, _) = listener.accept().await?;
        let mut buffer = vec![];
        stream.read_to_end(&mut buffer).await?;
        
        // Process the received frame
        let frame = imdecode(&buffer, videoio::IMREAD_COLOR)?;
        // ...
        // Do something with the frame
        // ...
    }
}
