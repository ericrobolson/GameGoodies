use std::f32::consts::PI;

use crate::oscillator::Oscillator;

pub struct Operator {
    carrier_frequency: f32,
}

pub struct Modulator {
    pub depth: f32,
    pub frequency: f32,
}

impl Operator {
    pub fn new(carrier_frequency: f32) -> Self {
        Self { carrier_frequency }
    }

    pub fn render(&self, t: f32, modulator: f32) -> f32 {
        crate::oscillator::sine(t, self.carrier_frequency, modulator)
    }
}
