use opencv::{highgui, prelude::*, videoio, Result};
use tokio::{net::TcpListener, io::{ AsyncWriteExt, BufReader, AsyncBufReadExt}};

#[tokio::main]
async fn main () -> Result<()>{
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2).unwrap();
    let mut frame = Mat::default();

    let (mut socket, _addr) = listener.accept().await.unwrap();
    println!("YO");

    let (reader, mut writer) = socket.split();
    
    let mut reader = BufReader::new(reader);
    
    // let mut line = String::new();
    
    loop {
        println!("GOING...");
        cam.read(&mut frame)?;

        // check whether VideoCapture still has frames to capture
        if !cam.grab().unwrap() {
            println!("Video processing finished");
            break
        }

        highgui::imshow("window", &frame)?;

        // let bytes_read = reader.read_line(&mut line).await.unwrap(); 
        
        // if bytes_read == 0 {
        //     break;
        // }
        
        writer.write_all(&frame.data_bytes().unwrap()).await.unwrap();
        println!("{:?}", frame.data_bytes());
        // line.clear();
    }

    Ok(())
} 


// fn main() -> Result<()> {
//     let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?;
//     let mut frame = Mat::default();
    
//     // moving wait_key(1) in the loop header allows for easy loop breaking if a condition is met (0 corresponds to 'ESC', 113 would be 'Q'
//     while highgui::wait_key(1)? < 0 {
//         cam.read(&mut frame)?;

//         // check whether VideoCapture still has frames to capture
//         if !cam.grab()? {
//             println!("Video processing finished");
//             break
//         }

//         highgui::imshow("window", &frame)?;

//     }

//     Ok(())
// }
