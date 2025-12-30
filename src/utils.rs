pub const AMPLITUDE_BOUND: f64 = 32768.0;
pub fn normalize(sample: i16) -> f64 {
    sample as f64 / AMPLITUDE_BOUND
}
pub fn denormalize(value: f64) -> i16 {
    (value * AMPLITUDE_BOUND) as i16
}
