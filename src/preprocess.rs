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
    let wav_samples = reader.samples::<i32>();
    let samples_len = wav_samples.len();

    println!("Actual number of samples in file: {}", samples_len);
    println!(
        "Filtered number of samples returned: {}",
        samples_len / resolution
    );
    
    let samples: Vec<Sample> = wav_samples.enumerate().map(|(i, s)| (i, s.unwrap())).collect();
    
    println!("Min: {}", samples.iter().map(|(_, s)| s).min().unwrap());
    println!("Max: {}", samples.iter().map(|(_, s)| s).max().unwrap());

    if resolution > 1 {
        samples.iter().filter(|(i, _)| i % resolution == 0).map(|s: &Sample| s.clone()).collect()
    } else {
        samples
    }
}