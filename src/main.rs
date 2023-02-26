use opencv::{
    core::{self, Mat, Vector},
    highgui::{self, VideoCapture},
    imgcodecs::{self, ImwriteFlags},
};
use std::{net::Ipv4Addr, thread, time};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a TCP listener on port 8080
    let mut listener = TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8080)).await?;
    
    // Start the video capture
    let mut capture = VideoCapture::new(0)?;
    capture.set(highgui::CAP_PROP_FRAME_WIDTH, 640.0)?;
    capture.set(highgui::CAP_PROP_FRAME_HEIGHT, 480.0)?;
    
    // Start capturing and processing frames
    loop {
        // Capture a frame from the video source
        let mut frame = Mat::default()?;
        capture.read(&mut frame)?;
        highgui::imshow("Camera", &frame)?;
        highgui::wait_key(10)?;
        
        // Convert the frame to a byte buffer
        let mut buffer = vec![];
        imgcodecs::imencode(".jpg", &frame, &mut buffer, &Vector::new())?;
        
        // Send the buffer over the network to all connected clients
        tokio::task::spawn(async move {
            while let Ok((mut stream, _)) = listener.accept().await {
                stream.write_all(&buffer).await.unwrap();
            }
        });
        
        // Sleep for a short period to limit the FPS
        thread::sleep(time::Duration::from_millis(33));
    }
}

#[tokio::main]
async fn receiver() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the streaming server and receive the frames
    let mut stream = TcpStream::connect("localhost:8080").await?;
    loop {
        let mut buffer = vec![];
        stream.read_to_end(&mut buffer).await?;
        
        // Process the received frame
        let frame = imgcodecs::imdecode(&buffer, imgcodecs::IMREAD_COLOR)?;
        // ...
        // Do something with the frame
        // ...
    }
}
