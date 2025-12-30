use crate::audio::traits::Effect;

pub struct TubeDistortion{
    pub rate: f64 // lower values more distortion
}

impl Effect for TubeDistortion{
    fn apply(&self, samples: Vec<f64>) -> Vec<f64>{
        samples
            .into_iter()
            .map(|s: f64| {
                    s - s.powi(3)/self.rate
                })
            .collect()
    }
}