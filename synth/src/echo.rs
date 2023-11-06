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
        let buffer = (0..(sample_rate / 2)).map(|_| 0.0).collect();
        let on = false;
        let volume = 0.5;
        let step = 0;
        let i = 0;
        let size = sample_rate as usize / 2;

        Self {
            buffer,
            on,
            volume,
            step,
            i,
            size,
        }
    }

    /// designed to accept floats greater then 1.0 and less then or equal to 5.0
    pub fn set_speed(&mut self, speed: Float) {
        self.step = ((1.0 / speed) * self.size as Float) as usize;
    }

    pub fn set_volume(&mut self, volume: Float) {
        self.volume = volume;
    }

    pub fn on(&mut self, on_off: bool) {
        if !on_off {
            // self.buffer = Buffer::new((SAMPLE_RATE / 2) as usize);
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
            // self.counter = (self.counter + 1) % self.size;
            self.i = (self.i + 1 + self.step) % self.size;
            // info!("i = {}", self.i);
            // info!("{}", self.i);
            echo
        } else {
            sample
        }
    }
}
