use std::time::Duration;
use rand::Rng;

use crate::Source;

/// Temporary copy to allow use of waves in main.r

/// 
/// Creates a triangle wave using 16 steps. This is a limitation of the NES and 
/// what gives it a unique sound
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESTriangleWave {
    freq: f32,
    num_sample: usize,
    steps: [f32;16],
}

impl NESTriangleWave {
    /// The frequency of the sine.
    #[inline]
    pub fn new(freq: f32) -> NESTriangleWave {
        NESTriangleWave {
            freq: freq,
            num_sample: 0,

            // The steps of the triangle wave
            steps: [-1.0, -0.86666, -0.73333, -0.6, -0.46666, -0.33333, -0.2, -0.06666, 0.06666, 0.2, 0.33333, 0.46666, 0.6, 0.73333, 0.86666, 1.0],
        }
    }
}
 
impl Iterator for NESTriangleWave {
    type Item = f32;

    ///
    /// This function imitates the NES triangle wave
    /// This could probably be generalized more
    /// TODO double check this is correct!
    /// 
    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let freq_ratio = self.freq / 48000.0;

        // Create a triangle wave, from 0-15 as float values
        let mut x = (((self.num_sample as f32 * 30.0) * freq_ratio) % 30.0) - 15.0;

        // Round the float values to indexes of an array corresponsing to the stepped triangle wave
        x = x.round();

        Some(self.steps[x as usize])
    }
}

impl Source for NESTriangleWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// 
/// Creates pulse wave
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESPulseWave {
    freq: f32,
    duty: f32,
    num_sample: usize,
}

impl NESPulseWave {
    /// The frequency of the sine.
    /// Duty is time of each pulse. 0.5 is a square wave
    #[inline]
    pub fn new(freq: f32, duty: f32) -> NESPulseWave {
        NESPulseWave {
            freq: freq,
            duty: duty,
            num_sample: 0,
        }
    }
}
 
impl Iterator for NESPulseWave {
    type Item = f32;

    ///
    /// This function imitates the NES Pulse wave
    /// 
    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        // Messy, should be cleaned up a bit
        // Divide the 48000 into segments of self.freq, then checks if the current sample
        // is less than half of that. If it is then return 0.0 otherwise return 1.0
        if (self.num_sample as f32 % (48000.0 / self.freq)) < (48000.0 / self.freq) * self.duty {
            return Some(0.0)
        }

        Some(1.0)
    }
}

impl Source for NESPulseWave {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// 
/// Creates noise using 16 steps. This is a limitation of the NES and 
/// what gives it a unique sound
/// Always has a rate of 48kHz and one channel.
/// 
#[derive(Clone, Debug)]
pub struct NESNoise {
    steps: [f32;16],
}

impl NESNoise {
    #[inline]
    pub fn new() -> NESNoise {
        NESNoise {
            // The steps that the noise can produce
            steps: [-1.0, -0.86666, -0.73333, -0.6, -0.46666, -0.33333, -0.2, -0.06666, 0.06666, 0.2, 0.33333, 0.46666, 0.6, 0.73333, 0.86666, 1.0],
        }
    }
}
 
impl Iterator for NESNoise {
    type Item = f32;

    ///
    /// This function imitates the NES Noise
    /// 
    #[inline]
    fn next(&mut self) -> Option<f32> {
        let x = rand::thread_rng().gen_range(0..self.steps.len());

        Some(self.steps[x])
    }
}

impl Source for NESNoise {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
