use hound;

pub type Sample = (usize, i32);
pub type Frame = (usize, u32);
pub type NormalizedFrame = (usize, f64);

pub type SampleVec = Vec<Sample>;
pub type FrameVec = Vec<Frame>;
pub type NormalizedFrameVec = Vec<NormalizedFrame>;


// TODO: This trait is shit.
trait VecUtil<T> {
    fn vecmin(&self) -> T;
    fn vecmax(&self) -> T;
    fn normalize(&self) -> Vec<f64>;
}

impl VecUtil<i32> for SampleVec {
    fn vecmin(&self) -> i32 {
        self.iter().map(|(_, s)| s).min().unwrap().to_owned()
    }

    fn vecmax(&self) -> i32 {
        self.iter().map(|(_, s)| s).max().unwrap().to_owned()
    }

    fn normalize(&self) -> Vec<f64> {
        todo!();
    }
}

impl VecUtil<u32> for FrameVec {
    fn vecmin(&self) -> u32 {
        self.iter().map(|(_, s)| s).min().unwrap().to_owned()
    }

    fn vecmax(&self) -> u32 {
        self.iter().map(|(_, s)| s).max().unwrap().to_owned()
    }

    fn normalize(&self) -> Vec<f64> {
        todo!()
    }
}

impl VecUtil<f64> for NormalizedFrameVec {
    fn vecmin(&self) -> f64 {
        todo!();
    }

    fn vecmax(&self) -> f64 {
        todo!();
    }

    fn normalize(&self) -> Vec<f64> {
        let min = self.vecmin();
        let max = self.vecmax();
        let div = (max - min) as f64;
        self.iter().map(|(_, v)| (v - min) as f64 / div).collect()
    }
}

pub fn normalize_frames(frames: FrameVec) -> NormalizedFrameVec {
    frames.normalize().iter().enumerate().map(|(l, r)| (l, *r)).collect()
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

    let samples: SampleVec = wav_samples
        .enumerate()
        .map(|(i, s)| (i, s.unwrap()))
        .collect();

    println!("Min: {}", samples.iter().map(|(_, s)| s).min().unwrap());
    println!("Max: {}", samples.iter().map(|(_, s)| s).max().unwrap());

    if resolution > 1 {
        samples
            .iter()
            .filter(|(i, _)| i % resolution == 0)
            .map(|s: &Sample| s.clone())
            .collect()
    } else {
        samples
    }
}
