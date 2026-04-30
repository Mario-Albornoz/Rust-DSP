mod audio;
mod effects;

use audio::{audio_connector::AudioConnector, io, pipeline::AudioPipeline};
use effects::{distortion::TubeDistortion, reverb::Reverb};

const CHORD_AUDIO_PATH: &str = "audioFiles/clean-electric-guitar-sustained-positivity.wav";
//const  SOLO_GUITAR_PATH: &str = "audioFiles/clean-electric-guitar-comfy-afternoon.wav";

fn main() {
    let loaded_input = io::load_input(CHORD_AUDIO_PATH);

    let pipeline = AudioPipeline::new()
        .add_effect(TubeDistortion { rate: 9.0 })
        .add_effect(Reverb {
            decay: 0.45,
            delay_rate_sec: 0.35,
            sample_rate: loaded_input.specs.sample_rate as f64,
            delay_sample_rate: 0.0,
            lowpass_smoothing_factor: 0.8,
            buffer_count: 4,
        });

    let processed = pipeline.process(loaded_input.samples);

    io::save_file("audioFiles/output.wav", processed, loaded_input.specs);

    let mut audio_connector = AudioConnector {};
    audio_connector.run();
}

//TODO:
//2. Alter the effects to process audio realtime (be able alter the audio by chunkz)
//3. Output transformer audio
//4. Alter effects so that the parameters are mutable during runtime, and customizable
//5. Add a loop function
//6. Connect camera to controler
