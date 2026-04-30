use hound::WavSpec;

//use crate::audio::traits::Effect;

pub struct Harmonizer {
    pub repeat_factor: usize,
    pub audio_specs: WavSpec,
}

//impl Effect for Harmonizer {
//    fn apply(&self, samples: Vec<f64>) -> Vec<f64> {
//        let audio_buffer: Vec<f64> = vec![0.0; self.audio_specs.sample_rate as usize];
//        let transformed_samples:Vec<f64>  = Vec::with_capacity(samples.len() * self.repeat_factor);
//        let audio_buffer_index: usize = 0usize;
//
//        for sample in samples{
//            let shifted: f64 = audio_buffer[audio_buffer_index];
//
//        }
//        return transformed_samples;
//    }
//}

