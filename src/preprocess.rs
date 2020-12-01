use hound;

pub type Sample = (usize, i32);
pub type Frame = (usize, u32);
pub type NormalizedFrame = (usize, f64);

pub trait VecUtil<T> {
    fn vecmin(&self) -> T;
    fn vecmax(&self) -> T;
    fn normalize(&self) -> Vec<f64>;
}

impl VecUtil<i32> for Vec<i32> {
    fn vecmin(&self) -> i32 {
        self.iter().min().unwrap().to_owned()
    }

    fn vecmax(&self) -> i32 {
        self.iter().max().unwrap().to_owned()
    }

    fn normalize(&self) -> Vec<f64> {
        let min = self.vecmin();
        let max = self.vecmax();
        let div = (max - min) as f64;
        self.iter().map(|v| (v - min) as f64 / div).collect()
    }
}

impl VecUtil<u32> for Vec<u32> {
    fn vecmin(&self) -> u32 {
        self.iter().min().unwrap().to_owned()
    }

    fn vecmax(&self) -> u32 {
        self.iter().max().unwrap().to_owned()
    }

    fn normalize(&self) -> Vec<f64> {
        let min = self.vecmin();
        let max = self.vecmax();
        let div = (max - min) as f64;
        self.iter().map(|v| (v - min) as f64 / div).collect()
    }
}

impl VecUtil<f64> for Vec<f64> {
    fn vecmin(&self) -> f64 {
        self.iter().map(|v| *v).fold(f64::NAN, f64::min)
    }

    fn vecmax(&self) -> f64 {
        self.iter().map(|v| *v).fold(f64::NAN, f64::max)
    }

    fn normalize(&self) -> Vec<f64> {
        let min = self.vecmin();
        let max = self.vecmax();
        let div = (max - min) as f64;
        self.iter().map(|v| (v - min) as f64 / div).collect()
    }
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

    let samples: Vec<Sample> = wav_samples
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
