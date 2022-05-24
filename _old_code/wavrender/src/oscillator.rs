use std::f32::consts::PI;

pub enum Oscillator {
    Sine {
        /// The frequency of the wave
        frequency: f32,
    },
    Square {
        frequency: f32,
    },
    Triangle {
        frequency: f32,
    },
    Saw {
        /// The frequency of the wave
        frequency: f32,
    },
}

pub fn sine(t: f32, freq: f32, modulator: f32) -> f32 {
    (t * freq * 2. * PI + modulator).sin()
}

fn square(t: f32, freq: f32) -> f32 {
    let v = (2. * (t * freq).floor() - (2. * t * freq).floor()) + 1.;
    v
}

impl Oscillator {
    pub fn frequency(&self) -> f32 {
        match self {
            Oscillator::Sine { frequency } => *frequency,
            Oscillator::Square { frequency } => *frequency,
            Oscillator::Triangle { frequency } => *frequency,
            Oscillator::Saw { frequency } => *frequency,
        }
    }

    pub fn sample(&self, t: f32) -> f32 {
        match self {
            Oscillator::Sine { frequency } => sine(t, *frequency, 0.),
            Oscillator::Square { frequency } => square(t, *frequency),
            Oscillator::Triangle { frequency } => {
                if *frequency == 0. {
                    return 0.;
                }
                let p = 1. / frequency;

                2. / PI * ((2. * PI * t / p).sin()).asin()
            }

            Oscillator::Saw { frequency } => {
                //
                let frequency = *frequency;

                if frequency == 0. {
                    return 0.;
                }
                let p = 1. / frequency;
                2. * (t / p - (0.5 + t / p).floor())
            }
        }
    }
}
