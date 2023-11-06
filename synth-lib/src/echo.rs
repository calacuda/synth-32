use log::info;

use super::Float;

/// fifo buffer of size SAMPLE_RATE that is used to get an "echo" effect.
pub struct Echo {
    buffer: Vec<Float>,
    pub on: bool,
    volume: Float,
    /// the speed of the echo
    step: usize,
    i: usize,
    size: usize,
}

impl Echo {
    pub fn new(sample_rate: u32) -> Self {
        // cut buffer size to half sample_rate if compiling for an original esp32 or
        // multiply by up to 1.75 for esp32-s3
        #[cfg(feature = "esp32s3")]
        let size = {
            info!(
                "running on an esp32-s3. increasing echo buffer size; allowing for longer echos..."
            );
            sample_rate as usize
        };
        #[cfg(not(feature = "esp32s3"))]
        let size = {
            info!("running on esp32. decreasing echo buffer to conserve memory...");
            (sample_rate as Float * 0.5) as usize
        };

        let buffer = (0..size).map(|_| 0.0).collect();
        let on = false;
        let volume = 0.5;
        let step = 0;
        let i = 0;

        Self {
            buffer,
            on,
            volume,
            step,
            i,
            size,
        }
    }

    /// designed to accept floats greater then 0.0 and less then or equal to 1.0 (arguably
    /// 0.5) when using 1.0 times sample_rate size.
    pub fn set_speed(&mut self, speed: Float) {
        // self.step = ((1.0 / speed) * self.size as Float) as usize;  // works for floats greater
        // then 1.0 and lest then 5.0 when using half sample_rate size.
        let size: Float = self.size as Float;
        self.step = (speed * size.powi(2)) as usize;
    }

    pub fn set_volume(&mut self, volume: Float) {
        self.volume = volume;
    }

    pub fn on(&mut self, on_off: bool) {
        if on_off {
            // clears the buffer so we dont hear the echo of what was played before the echo was
            // turned on.
            for i in 0..self.buffer.len() {
                self.buffer[i] = 0.0;
            }

            self.i = 0;
        }

        self.on = on_off;
    }

    pub fn toggle(&mut self) {
        self.on = !self.on;
    }

    pub fn pop(&mut self, sample: Float) -> Float {
        if self.on {
            let echo = ((self.buffer[self.i] * self.volume) + sample) * 0.5;
            self.buffer[self.i] = echo;
            self.i = (self.i + 1 + self.step) % self.size;
            echo
        } else {
            sample
        }
    }
}
