use crate::audio::traits::Effect;

pub struct Reverb {
    pub decay: f64, //higher values longer reverb tail
    pub delay_rate_sec: f64, //delay between echoes, higher values more spaced out echoes
    pub sample_rate : f64, //sample rate used to conver seconds to samples
    pub delay_sample_rate: f64, //Determine how far back the echoes come from 
    pub lowpass_smoothing_factor: f64, //Higher values high frequencies decay faster
    pub buffer_count: i16
}

impl Effect for Reverb{
    fn apply(&self, samples: Vec<f64>) -> Vec<f64> {
        let delay_samples = (self.delay_rate_sec * self.sample_rate) as usize;
        let mut delay_buffer: Vec<f64> = vec![0.0; 
            if self.delay_sample_rate > 0.0{
                self.delay_sample_rate as usize
            } else {
                delay_samples
            }];

        let mut delay_buffer_index: usize = 0usize;

        let mut transformed_samples: Vec<f64> = Vec::with_capacity(samples.len());

        for sample in samples{
            let delayed = delay_buffer[delay_buffer_index];
            
            let filtered_feedback = self.handle_buffers( self.buffer_count, self.decay, delayed);
            let output = (sample + filtered_feedback).clamp(-1.0, 1.0);

            delay_buffer[delay_buffer_index] = sample + filtered_feedback * self.decay;
            delay_buffer_index = (delay_buffer_index + 1) % delay_buffer.len();
            transformed_samples.push(output);
        };

        return transformed_samples
    }
}

impl Reverb {
    fn handle_buffers(&self, buffer_count: i16, decay: f64, delayed_sample:f64) -> f64{
        if buffer_count == 0
        {
            return delayed_sample * decay
        } else {
            let next_feedback = self.lowpass_smoothing_factor * delayed_sample + (1.0 - self.lowpass_smoothing_factor) * delayed_sample * 0.5;

            let current = next_feedback * decay;
            let deeper = self.handle_buffers( buffer_count - 1, decay * 0.3, current);

            return current + deeper * 0.5;
        }

    }
}