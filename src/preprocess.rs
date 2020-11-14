use hound;

pub type Sample = (usize, i32);
pub type Frame = (usize, u32);
pub type NormalizedFrame = (usize, f64);

pub fn normalize_frames(frames: Vec<Frame>) -> Vec<NormalizedFrame> {
    let min = frames.iter().map(|(_, s)| s).min().unwrap();
    let max = frames.iter().map(|(_, s)| s).max().unwrap();
    let div = (max - min) as f64;
    frames
        .iter()
        .map(|(i, v)| (*i, (v - min) as f64 / div))
        .collect()
}

pub fn get_indexed_samples(filename: &str, resolution: usize) -> Vec<Sample> {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let all_samples = reader.samples::<i32>();
    let number = all_samples.len();

    println!("Actual number of samples in file: {}", number);
    println!(
        "Filtered number of samples returned: {}",
        number / resolution
    );

    let temp = all_samples.enumerate().map(|(i, s)| (i, s.unwrap() as i32));

    if resolution > 1 {
        temp.filter(|(i, _)| i % resolution == 0).collect()
    } else {
        temp.collect()
    }
}