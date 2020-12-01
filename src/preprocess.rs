use hound;

pub type Sample = (usize, i32);
pub type Frame = (usize, u32);
pub type NormalizedFrame = (usize, f64);

pub type SampleVec = Vec<Sample>;
pub type FrameVec = Vec<Frame>;
pub type NormalizedFrameVec = Vec<NormalizedFrame>;

trait MinMax<T> {
    fn vecmin(&self) -> T;
    fn vecmax(&self) -> T;
}

impl MinMax<i32> for SampleVec {
    fn vecmin(&self) -> i32 {
        self.iter().map(|(_, s)| s).min().unwrap().to_owned()
    }

    fn vecmax(&self) -> i32 {
        self.iter().map(|(_, s)| s).max().unwrap().to_owned()
    }
}

impl MinMax<u32> for FrameVec {
    fn vecmin(&self) -> u32 {
        self.iter().map(|(_, s)| s).min().unwrap().to_owned()
    }

    fn vecmax(&self) -> u32 {
        self.iter().map(|(_, s)| s).max().unwrap().to_owned()
    }
}

// impl MinMax<f64> for NormalizedFrameVec {
//     fn vecmin(&self) -> f64 {
//         self.iter().map(|(_, s)| s).min().unwrap().to_owned()
//     }

//     fn vecmax(&self) -> f64 {
//         self.iter().map(|(_, s)| s).max().unwrap().to_owned()
//     }
// }

pub fn normalize_frames(frames: FrameVec) -> NormalizedFrameVec {
    //let min = frames.iter().map(|(_, s)| s).min().unwrap();
    //let max = frames.iter().map(|(_, s)| s).max().unwrap();
    let min = frames.vecmin();
    let max = frames.vecmax();
    let div = (max - min) as f64;
    frames
        .iter()
        .map(|(i, v)| (*i, (v - min) as f64 / div))
        .collect()
}

pub fn get_indexed_samples(filename: &str, resolution: usize) -> SampleVec {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let wav_samples = reader.samples::<i32>();
    let samples_len = wav_samples.len();

    println!("Actual number of samples in file: {}", samples_len);
    println!(
        "Filtered number of samples returned: {}",
        samples_len / resolution
    );
    
    let samples: SampleVec = wav_samples.enumerate().map(|(i, s)| (i, s.unwrap())).collect();
    
    println!("Min: {}", samples.iter().map(|(_, s)| s).min().unwrap());
    println!("Max: {}", samples.iter().map(|(_, s)| s).max().unwrap());

    if resolution > 1 {
        samples.iter().filter(|(i, _)| i % resolution == 0).map(|s: &Sample| s.clone()).collect()
    } else {
        samples
    }
}