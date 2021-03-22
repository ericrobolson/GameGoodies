use hound;
use operator::{Modulator, Operator};
use std::f32::consts::PI;
use std::i16;
use std::io::BufReader;

mod envelope;
use envelope::Envelope;
mod oscillator;
use oscillator::Oscillator;

mod operator;

fn main() {
    let sample_rate = 44100;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();

    let duration_seconds = 8;

    let mut envelope = Envelope::new(2., 2., 0.5, 2., sample_rate as f32);
    envelope.on();
    let mut wobble = Envelope::new(1., 1., 0.666, 1., sample_rate as f32);
    wobble.on();
    for t in (0..sample_rate * duration_seconds).map(|x| x as f32 / 44100.0) {
        if t >= 4. {
            envelope.off();
        }

        // Example modulator
        let modulator = Oscillator::Sine { frequency: 130. }.sample(t);
        let mod2 = Oscillator::Square { frequency: 65. }.sample(t);
        let r = Operator::new(65.).render(t, 8. * modulator * wobble.tick());

        let e = envelope.tick();

        let sample = (mod2 + r) * e;

        let const_amplitude = i16::MAX as f32;
        writer
            .write_sample((sample * const_amplitude) as i16)
            .unwrap();
    }
    writer.finalize().unwrap();
}

/// Maps one range to another
pub fn map_range<Num>(
    input: Num,
    input_start: Num,
    input_end: Num,
    output_start: Num,
    output_end: Num,
) -> Num
where
    Num: Copy
        + std::ops::Add<Output = Num>
        + std::ops::Sub<Output = Num>
        + std::ops::Div<Output = Num>
        + std::ops::Mul<Output = Num>,
{
    output_start + ((input - input_start) * (output_end - output_start)) / (input_end - input_start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_active_0_returns_false() {
        let rem = 4 % 3;
        let r = rem as f32;

        let q = f32::rem_euclid(4.0, 3.);

        assert_eq!(r, q);
    }
}
