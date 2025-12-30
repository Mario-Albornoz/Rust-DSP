use std::f32::consts::PI;

use crate::audio::traits::Effect;

pub struct Harmonizer{
    pub vol: f64,
}

impl Effect for Harmonizer{
    fn apply(&self, samples: Vec<f64>) -> Vec<f64> {
        samples
            .into_iter()
            .map(|s: f64| s + (PI as f64 * s as f64) * self.vol)
            .collect()
    }
}