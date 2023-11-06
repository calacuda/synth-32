use super::Float;

#[derive(Clone, Debug)]
pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<Float>,
    index: Float,
    index_increment: Float,
}

impl WavetableOscillator {
    pub fn new(sample_rate: u32, wave_table: Vec<Float>) -> Self {
        Self {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: Float) {
        self.index_increment =
            frequency * self.wave_table.len() as Float / self.sample_rate as Float;
    }

    pub fn get_sample(&mut self) -> Float {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as Float;
        sample
    }

    fn lerp(&self) -> Float {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as Float;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}
