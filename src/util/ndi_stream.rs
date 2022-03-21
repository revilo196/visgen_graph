use std::sync::{Arc};

use ndi;  //custom fork/branch with fixes for sending NDI
use image::{ImageBuffer, Rgba};
use crossbeam::queue::ArrayQueue;

type NdiImageBuffer = ImageBuffer<Rgba<u8>, Vec<u8>>;
type NdiTimedFrame = (Box<NdiImageBuffer>, i64);

/// NDI Video Stream
pub struct NdiStream {
    send : ndi::Send,
    framerate: i32,
    queue : Arc<ArrayQueue<NdiTimedFrame>>,
    frame : Option<ndi::VideoData>,
}


impl NdiStream 
{   
    /// create new NdiStream
    pub fn new(name: String, framerate: i32) -> Self {
        Self {
            send: ndi::SendBuilder::new().ndi_name(name).clock_video(true).build().expect("error creating NDI sender"),
            framerate,
            queue: Arc::new(ArrayQueue::new(2)),
            frame: None,
        }
    }

    /// send an image buffer in the stream
    pub fn send_image(&mut self, image: Box<ImageBuffer<Rgba<u8>, Vec<u8>>>, timecode: i64)   { 

        let width = image.width();
        let height = image.height();
        let mut flat = image.into_flat_samples();
        let stride = flat.strides_cwh().2 as i32;
        let fourcc = ndi::FourCCVideoType::RGBA;
        let buffer = flat.samples.as_mut_ptr();
        
        //println!("Sending Frame: {:?}", (width,height,fourcc,self.framerate,timecode,stride));

        self.frame = Some(ndi::VideoData::from_buffer(width as i32, height as i32, fourcc, self.framerate, ndi::FrameFormatType::Progressive, timecode, stride, buffer));
        if let Some(video_data) = &self.frame {
            self.send.send_video_async(video_data);
        }
    }

    /// receives and async image_buffer and sends it in the stream
    pub fn send_video_from_queue(&mut self) {
        if let Some((img, time)) = self.queue.pop()  {
                self.send_image(img, time); // ToDo make it possible to async this
        }
    }


    pub fn update_snapshot(&mut self, snapshot: nannou::wgpu::TextueSnapshot, timecode: i64) {
        // send the last queued image in the stream;
        self.send_video_from_queue();
        let qu = self.queue.clone();

        //take a snapshot and send it to ndi_stream
        //  snapshot.read is async the frame is only queued and send in the next pass
        snapshot.read(move |result| {
        let image = result.expect("faild to map texture").to_owned();
        qu.push((Box::new(image), timecode)).ok();
        }).unwrap();
    }
}

