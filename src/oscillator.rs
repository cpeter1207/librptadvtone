//! The existing sine and phase arithmetic shared with Morse rendering.

/// One 48 kHz sine oscillator with explicit phase reset.
pub(crate) struct Oscillator {
    phase: f32,
    step: f32,
    amplitude: f32,
}

impl Oscillator {
    /// Prepare frequency and amplitude outside the audio callback.
    pub(crate) fn new(frequency_hz: f32, amplitude: f32) -> Self {
        Self {
            phase: 0.0,
            step: frequency_hz / 48_000.0,
            amplitude,
        }
    }

    /// Reset the phase at a keyed element boundary.
    pub(crate) fn reset(&mut self) {
        self.phase = 0.0;
    }

    /// Fill the requested output span without allocating.
    pub(crate) fn render(&mut self, output: &mut [f32]) {
        for sample in output {
            *sample = self.amplitude * (core::f32::consts::TAU * self.phase).sin();
            self.phase = (self.phase + self.step).fract();
        }
    }
}
