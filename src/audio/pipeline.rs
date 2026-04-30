use super::traits::Effect;

pub struct AudioPipeline{
    //dyn refers to a dynamic effect, since we don't know at compile time which effects will be applied
    //Box allows us to declare our vector because box will only hold a pointer to where effects will be in the heap.
    // Therefore it is fixed in size and will allow us to initialize a vec
    effects : Vec<Box<dyn Effect>>
}

impl AudioPipeline {
    pub fn new() -> Self {
        //create a new AudioPipeline struct with an empty vector to hold our effects 
        Self { effects: vec![] }
    }

    //Declare effect type as type any thta implements effect and is static lifetime to avoid pointing a freed effect 
    pub fn add_effect<EffectType: Effect + 'static>(mut self, effect: EffectType) -> Self {
        self.effects.push(Box::new(effect));
        self
    }

    pub fn process(&self, samples: Vec<f64>)-> Vec<f64> {
        self.effects.iter().fold(samples, |acc, eff| eff.apply(acc))
    }
}
