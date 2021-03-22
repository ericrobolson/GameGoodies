use crate::map_range;

//http://hackmeopen.com/2011/12/synth-diy-software-for-generating-adsr-envelopes/

pub struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    state: State,
    inner_time: f32,
    sample_delta: f32,
}

#[derive(PartialEq)]
enum State {
    Off,
    Attack,
    Decay,
    Sustain,
    Release,
}

impl Envelope {
    pub fn new(attack: f32, decay: f32, sustain: f32, release: f32, sample_rate: f32) -> Self {
        let sustain = sustain.min(1.);

        Self {
            attack,
            decay,
            sustain,
            release,
            inner_time: 0.,
            state: State::Off,
            sample_delta: 1. / sample_rate,
        }
    }

    pub fn on(&mut self) {
        self.inner_time = self.attack;
        self.state = State::Attack;
    }

    pub fn tick(&mut self) -> f32 {
        self.inner_time -= self.sample_delta;
        match self.state {
            State::Off => 0.,
            State::Attack => {
                // Attack goes from 0 .. 1
                if self.inner_time <= 0. {
                    self.state = State::Decay;
                    self.inner_time = self.decay;
                    1.
                } else {
                    let delta = self.inner_time / self.attack;
                    let attack = 1. - delta;
                    attack
                }
            }
            State::Decay => {
                // Decay goes from 1 .. sustain
                if self.inner_time <= 0. {
                    self.state = State::Sustain;
                    self.inner_time = 0.;
                    self.sustain
                } else {
                    let delta = self.inner_time / self.decay;
                    let decay = 1. - (1. - delta);
                    map_range(decay, 1., 0., 1., self.sustain)
                }
            }
            State::Sustain => self.sustain,
            State::Release => {
                // Release goes from sustain .. 0
                if self.inner_time <= 0. {
                    self.state = State::Off;
                    self.inner_time = 0.;
                    0.
                } else {
                    let delta = self.inner_time / self.release;
                    let release = 1. - (1. - delta);

                    self.sustain * release
                }
            }
        }
    }

    pub fn off(&mut self) {
        if self.state != State::Release && self.state != State::Off {
            self.state = State::Release;
            self.inner_time = self.release;
        }
    }
}
