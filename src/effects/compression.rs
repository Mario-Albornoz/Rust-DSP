use crate::audio::traits::Effect;

pub struct Compression{
    threshold: f64,
    ratio: f64
}

impl Effect for Compression{
    fn apply(&self, samples: Vec<f64>) -> Vec<f64> {
        samples
            .into_iter()
            .map(|s: f64| self.threshold + ((s - self.threshold)/self.ratio))
            .collect() 
        }   
}