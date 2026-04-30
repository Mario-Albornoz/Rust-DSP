use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioConnector {}

impl AudioConnector {
    pub fn run(&mut self) {
        self.start_input_stream()
            .expect("error setting up input device");
    }

    fn start_input_stream(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let host = cpal::default_host();

        let input_device = host
            .default_input_device()
            .expect("failed to find input device");

        println!("Input device: {}", input_device.name()?);

        let config = input_device
            .default_input_config()
            .expect("Failed to get default input/output config");

        let config: cpal::StreamConfig = config.into();

        let stream = input_device.build_input_stream(
            &config,
            move |data: &[f32], _| {
                for sample in data {
                    println!("Sample: {}", sample);
                }
            },
            move |err| {
                eprintln!("erro in stream {err}");
            },
        )?;

        stream.play()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        Ok(())
    }
}
