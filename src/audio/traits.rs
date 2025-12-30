pub trait Effect {
    fn apply(&self, samples: Vec<f64>) -> Vec<f64>;
}