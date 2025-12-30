use hound::{ WavReader, WavSpec, WavWriter};
use std::{fs::File, io::{BufReader, BufWriter}};


const AMPLITUDE_BOUND : f64 = 32768.0;

pub struct LoadInput {
    pub samples: Vec<f64>,
    pub specs : WavSpec,
}

pub fn load_input(path: &str) -> LoadInput{

    let mut reader:WavReader<BufReader<File>> = hound::WavReader::open(path).unwrap();

    LoadInput{
        samples : reader.samples::<i16>().map(|s| {s.unwrap() as f64 / AMPLITUDE_BOUND }).collect(), 
        specs : reader.spec()
    }
}

pub fn save_file(path: &str, samples: Vec<f64>, specs:WavSpec) {
    
    let mut writer: WavWriter<BufWriter<File>> = hound::WavWriter::create(path, specs).unwrap();

    for s in samples {
        let scaled = (s * AMPLITUDE_BOUND).clamp(-AMPLITUDE_BOUND, AMPLITUDE_BOUND - 1.0);
        writer.write_sample(scaled as i16).unwrap();
    }
    writer.finalize().unwrap();

}