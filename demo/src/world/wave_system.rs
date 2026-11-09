use crate::timer::Timer;

pub struct WaveSystem {
    pub timer: Timer,
    pub current_wave: usize,
}

impl WaveSystem {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(10.0),
            current_wave: 1,
        }
    }
}
