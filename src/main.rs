use image::gif::{GifDecoder, GifEncoder};
use image::AnimationDecoder;
use std::fs::File;
use std::time;
fn main() {
    let mut start = time::Instant::now();
    let file_in = File::open("foo.gif").unwrap();
    let decoder = GifDecoder::new(file_in).unwrap();
    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding gif");
    println!("Decoding: {}ms", start.elapsed().as_millis());
    start = time::Instant::now();
    let file_out = File::create("out.gif").unwrap();
    println!("File Creation: {}ms", start.elapsed().as_millis());
    start = time::Instant::now();
    let mut encoder = GifEncoder::new(file_out);
    encoder.encode_frames(frames.into_iter()).unwrap();
    println!("Gif Encoding: {}ms", start.elapsed().as_millis());

}
